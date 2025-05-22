# EBS - Emergency Brake System

## Introduzione

La scheda dell'*Emergency Brake System* (***EBSCU***)gestisce la parte terminale dell'*Autonomous System Brake* (*ASB*), occupandosi di:
- fare l'*initial checkup* per garantire il corretto funzionamento del freno di emergenza
- monitorare la pressione dei serbatoi di aria compressa utilizzati per l'attivazione dei freni
-  attuare i freni idraulici in *finishing run*

## Indice <a id="indice"></a>

- [Introduzione](#introduzione)
- [Quando usa Autonomous Brake?](#quando-usa-autonomous-brake)
  - [Inizio Run](#inizio-run)
  - [Fine Run](#fine-run)
- [Attuazione freni idraulici](#attuazione-freni-idraulici)
- [Rilascio freni](#rilascio-freni)
- [Monitoraggio pressione](#monitoraggio-pressione)
- [ASB Check](#asb-check)
- [Valori pressioni](#valori-pressioni)
---
## Quando usa Autonomous Brake?

Durante ogni run *driverless*, il freno viene utilizzato:
1. **Attivazione dell'*ASMS***: *SDC* è aperto, quindi siamo in EB
2. ***ASB* Check valve 1**
3. ***ASB* Check valve 2**
4. **Pre-run**: prima di partire la macchina è frenata
5. **Finish run**: per garantire standing still

In `1.` vengono aperte entrambe le valvole, negli altri punti una sola.
Quindi ogni tank dovrà garantire almeno 3 impulsi.


### Inizio Run

- *EBS* *engaged*
- *ASB check*
- *Engage brakes*

### Fine Run

Quando lo `ASstatus`aggiornato in `finishing_run`[<sup>3</sup>](#nota3), si procede all'attivazione dei freni. Viene aperta una sola valvola.

---
## Attuazione freni idraulici

Due serbatoi di aria compressa sono collegati, ciascuno a un pistone, tramite valvole azionate elettricamente. L'apertura delle valvole provoca l'estensione del pistone, il quale attua i freni.

Sulla scheda sono presenti due pin **GPIO**: uno per il freno, che, all'attivazione della scheda, viene impostato su `HIGH`.

Le valvole rimangono chiuse fintanto che sia l'*SDC* risulta chiuso sia i relativi pin sono in `HIGH`; pertanto, sia l'apertura dell'*SDC* sia lo stato `LOW` dei pin comportano l'attivazione dei freni.


---
## Rilascio freni

Si distinguono due casi:

1. **Freno del sistema autonomo:** il rilascio dei freni avviene quando i pin della scheda tornano su `HIGH`. L'***EBSCU*** si occupa direttamente <u>solo</u> di questa.
2. **Frenata di emergenza:** se la centralina o un altro nodo dell'*SDC* apre il circuito, il rilascio deve essere effettuato manualmente tramite i *brake release*, apposite maniglie predisposte a questo scopo.

---
## Monitoraggio pressione

La scheda legge i valori inviati dai due sensori di pressione posizionati nei serbatoi tramite due pin analogici. I valori vengono filtrati tramite *moving average* (buffer di venti valori[<sup>1</sup>](#nota1)), convertiti in pressione e inviati periodicamente (ogni 100 ms[<sup>1</sup>](#nota1)) in CAN mediante il messaggio `EbsTank`[<sup>2</sup>](#nota2). Quest'ultimo include anche informazioni sulla salute dei sensori e segnala se la pressione dei serbatoi è inferiore al *threshold*[<sup>2</sup>](#nota2).

---

## ASB check

---

## Valori pressioni

### Tank

8 Bar è *threshold* al di sotto il quale non possiamo andare.
10 Bar è il massimo consentito da regolamento.

### Pistone idraulico freni

Attualmente abbiamo un 60/40 *front*/*rear*. ***brake balance***

Con una pressione pneumatica manuale di 9.5 Bar otteniamo:
- **Front**: 33.2 Bar
- **Rear** : 28.4 Bar

La *funzione di trasferimento* traduce la proporzionalità diretta tra *pressione idraulica* e *pressione pneumatica*.

$$ pi_f = k_f \cdot  pp $$
$$ pi_r = k_r \cdot  pp $$

---

### Codice

La vita dell'*EBS* si sviluppa in 4 fasi:
0. ***Inactive***: mission `None`o `Manual`
1. ***AS Off***: mission `Driverless`
2. ***ASB Check***
3. ***AS Ready***
4. ***Run***



---

## Note

<a id="nota1"></a>
[<sup>1</sup>] - Questi valori possono variare. [Torna all'indice](#indice)

<a id="nota2"></a>
[<sup>2</sup>] - Nome da verificare, potrebbe subire modifiche. Il messaggio contiene valori di pressione da 5 a 10, con una sensibilità di 0.125 espressa in Bar; 6 Bar rappresentano il threshold. [Torna all'indice](#indice)

<a id="nota3"></a>
[<sup>3</sup>] - *CULO* dichiara lo *status* della macchina con il messaggio `CarMissionStatus` in *can 2*.
