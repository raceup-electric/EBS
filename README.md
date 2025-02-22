# EBS - Emergency Brake System

## Introduzione

La scheda dell'*Emergency Brake System* gestisce la parte terminale dell'*Autonomous System Brake* (*ASB*), occupandosi di:
- attuare i freni idraulici in modalità *driverless*
- monitorare la pressione dei serbatoi di aria compressa utilizzati per l'attivazione dei freni

## Indice <a id="indice"></a>

- [Introduzione](#introduzione)
- [Attuazione freni idraulici](#attuazione-freni-idraulici)
- [Rilascio freni](#rilascio-freni)
- [Monitoraggio pressione](#monitoraggio-pressione)
- [Quando usa Autonomous Brake?](#quando-usa-autonomous-brake)
  - [Inizio Run](#inizio-run)
  - [Fine Run](#fine-run)

## Attuazione freni idraulici

Due serbatoi di aria compressa sono collegati, ciascuno a un pistone, tramite valvole azionate elettricamente. L'apertura delle valvole provoca l'estensione del pistone, il quale attua il freno idraulico corrispondente.

Sulla scheda sono presenti due pin **GPIO**: uno per il freno, che, all'attivazione della scheda, viene impostato su `HIGH`.

Le valvole rimangono chiuse fintanto che sia l'*SDC* risulta chiuso sia i relativi pin sono in `HIGH`; pertanto, sia l'apertura dell'*SDC* sia l'impostazione dei pin su `LOW` comportano l'attivazione dei freni.

## Rilascio freni

Si distinguono due casi:

1. **Freno del sistema autonomo:** il rilascio dei freni avviene quando i pin della scheda tornano su `HIGH`.
2. **Frenata di emergenza:** se la centralina o un altro nodo dell'*SDC* apre il circuito, il rilascio deve essere effettuato manualmente tramite i *brake release*, apposite maniglie predisposte a questo scopo.

## Monitoraggio pressione

La scheda legge i valori inviati dai due sensori di pressione posizionati nei serbatoi tramite due pin analogici. I valori vengono filtrati tramite *moving average* (buffer di venti valori[<sup>1</sup>](#nota1)), convertiti in pressione e inviati periodicamente (ogni 100 ms[<sup>1</sup>](#nota1)) in CAN mediante il messaggio `EbsTank`[<sup>2</sup>](#nota2). Quest'ultimo include anche informazioni sulla salute dei sensori e segnala se la pressione dei serbatoi è inferiore al *threshold*[<sup>3</sup>](#nota3).

## Quando usa Autonomous Brake?

Durante ogni run *driverless*, il freno viene utilizzato due volte: all'inizio del run e alla sua fine (con l'eccezione dei *test EBS*).

### Inizio Run

Quando la scheda riceve il messaggio `ESBCheck` dalla centralina, abbassa i due pin per attivare il freno e, successivamente, invia il messaggio `EBSCheck` con l'esito della procedura. Questa azione ha il duplice scopo di verificare il corretto funzionamento dell'*ASB* e di garantire lo *standing still* all'inizio del run, condizione necessaria per raggiungere lo stato di *ready to drive*.

### Fine Run

Alla ricezione del messaggio `EndRun`[<sup>3</sup>](#nota3), si procede all'attivazione dei freni.

---

## Note

<a id="nota1"></a>
[<sup>1</sup>] - Questi valori possono variare. [Torna all'indice](#indice)

<a id="nota2"></a>
[<sup>2</sup>] - Nome da verificare, potrebbe subire modifiche. Il messaggio contiene valori di pressione da 5 a 10, con una sensibilità di 0.125 espressa in Bar; 6 Bar rappresentano il threshold. [Torna all'indice](#indice)

<a id="nota3"></a>
[<sup>3</sup>] - Messaggio non ancora incluso nel file DBC. [Torna all'indice](#indice)
