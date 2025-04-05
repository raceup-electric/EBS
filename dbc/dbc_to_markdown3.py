#!/usr/bin/env python3
import sys
import os
import cantools
import re

def safe_anchor(text):
    """
    Crea un anchor "sicuro" in stile markdown:
    Converte in minuscolo, sostituisce spazi e caratteri non alfanumerici con -
    """
    # Converte in minuscolo
    anchor = text.lower()
    # Sostituisce spazi e caratteri speciali con -
    anchor = re.sub(r'[^a-z0-9]+', '-', anchor)
    # Rimuove eventuali - iniziali o finali
    anchor = anchor.strip('-')
    return anchor

def generate_index_table(db):
    table = "| Nome | ID | DLC | Note |\n"
    table += "| ---- | -- | --- | ---- |\n"
    for msg in db.messages:
        anchor = safe_anchor(msg.name)
        note = msg.comment if msg.comment else ""
        # Visualizza l'ID sia in decimale che in esadecimale
        id_str = f"{msg.frame_id} (0x{msg.frame_id:x})"
        table += f"| [{msg.name}](#{anchor}) | {id_str} | {msg.length} | {note} |\n"
    return table

def generate_signals_table(msg):
    if not msg.signals:
        return "_Nessun segnale definito_\n\n"
    
    table = ("| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment | Enum |\n" +
             "| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- | ---- |\n")
    for signal in msg.signals:
        # Prepariamo il range se definiti
        range_str = ""
        if signal.minimum is not None and signal.maximum is not None:
            range_str = f"[{signal.minimum}, {signal.maximum}]"
        comment_str = signal.comment if signal.comment else ""
        unit_str = signal.unit if signal.unit else ""
        # Prepariamo l'enum se presenti (dalla conversione)
        enum_str = ""
        if signal.conversion and hasattr(signal.conversion, "choices") and signal.conversion.choices:
            # Creiamo una stringa con "chiave: valore" separati da "; "
            enum_items = [f"{key}: {val}" for key, val in signal.conversion.choices.items()]
            enum_str = "; ".join(enum_items)
        
        table += (f"| {signal.name} | {signal.start} | {signal.length} | {signal.byte_order} "
                  f"| {signal.is_signed} | {unit_str} | {range_str} | {comment_str} | {enum_str} |\n")
    return table

def generate_message_section(msg):
    anchor = safe_anchor(msg.name)
    section = f"## {msg.name}\n\n"
    # Esplicitiamo i dettagli principali
    section += f"**ID:** {msg.frame_id} (0x{msg.frame_id:x})\n\n"
    section += f"**DLC:** {msg.length}\n\n"
    if msg.comment:
        section += f"**Note:** {msg.comment}\n\n"
    
    section += "### Segnali\n\n"
    section += generate_signals_table(msg)
    section += "\n---\n\n"
    return section

def generate_markdown(db):
    md = "# Documentazione DBC\n\n"
    md += "## Indice Messaggi\n\n"
    md += generate_index_table(db)
    md += "\n---\n\n"
    
    # Sezione per ogni messaggio
    for msg in db.messages:
        md += generate_message_section(msg)
    
    # Se desideri aggiungere una sezione finale per eventuali note generali,
    # la puoi aggiungere qui.
    # Esempio:
    # md += "## Note Generali\n\n"
    # md += "Qui puoi inserire note aggiuntive...\n"
    return md

def main():
    if len(sys.argv) < 3:
        print("Usage: python dbc_to_markdown.py <input_dbc_file> <output_markdown_file>")
        sys.exit(1)
    
    input_dbc = sys.argv[1]
    output_md = sys.argv[2]
    
    try:
        db = cantools.database.load_file(input_dbc)
    except Exception as e:
        print(f"Errore nel caricamento del file DBC: {e}")
        sys.exit(1)
    
    markdown_content = generate_markdown(db)
    
    output_dir = os.path.dirname(os.path.abspath(output_md))
    if not os.path.isdir(output_dir):
        os.makedirs(output_dir, exist_ok=True)
    
    with open(output_md, "w", encoding="utf-8") as f:
        f.write(markdown_content)
    
    print(f"File Markdown generato: {output_md}")

if __name__ == "__main__":
    main()
