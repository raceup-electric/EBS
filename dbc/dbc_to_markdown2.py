#!/usr/bin/env python3
import sys
import cantools

def generate_markdown(db):
    md = "# Indice dei Messaggi\n\n"
    # Crea l'indice: ogni voce è un link interno che punta al sottotitolo del messaggio.
    for msg in db.messages:
        # Utilizzo la sintassi [[#NomeMessaggio]] per linkare allo heading "## NomeMessaggio"
        md += f"- [[#{msg.name}]] (ID: {msg.frame_id}, DLC: {msg.length})\n"
    
    md += "\n---\n\n"
    
    # Per ogni messaggio, aggiungo una sezione con heading e una tabella dei segnali
    for msg in db.messages:
        md += f"## {msg.name}\n\n"
        md += f"**ID:** {msg.frame_id} (0x{msg.frame_id:x})\n\n"
        md += f"**DLC:** {msg.length}\n\n"
        if msg.comment:
            md += f"**Commento:** {msg.comment}\n\n"
        
        # Se vuoi, puoi aggiungere qui un blocco che rappresenta graficamente il layout (frame)
        # del messaggio. Ad esempio, si potrebbe inserire l'output di as_dbc_string() in un blocco di codice:
        # md += "```dbc\n" + msg.dbc.as_dbc_string() + "\n```\n\n"
        #
        # In questo esempio ci limitiamo alla tabella dei segnali.
        md += "### Segnali\n\n"
        if not msg.signals:
            md += "_Nessun segnale definito_\n\n"
        else:
            # Intestazione della tabella Markdown
            md += (
                "| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |\n"
                "| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |\n"
            )
            for signal in msg.signals:
                # Prepara i valori per range e commento
                range_str = ""
                if signal.minimum is not None and signal.maximum is not None:
                    range_str = f"[{signal.minimum}, {signal.maximum}]"
                comment_str = signal.comment if signal.comment else ""
                unit_str = signal.unit if signal.unit else ""
                md += (
                    f"| {signal.name} | {signal.start} | {signal.length} | {signal.byte_order} "
                    f"| {signal.is_signed} | {unit_str} | {range_str} | {comment_str} |\n"
                )
            md += "\n"
        md += "---\n\n"
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
    
    # Scrive il file di output; se la directory non esiste, tenta di crearla
    import os
    output_dir = os.path.dirname(output_md)
    if output_dir and not os.path.isdir(output_dir):
        os.makedirs(output_dir, exist_ok=True)
    
    with open(output_md, "w", encoding="utf-8") as f:
        f.write(markdown_content)
    
    print(f"File Markdown generato: {output_md}")

if __name__ == "__main__":
    main()
