#!/usr/bin/env python3
import sys
import cantools

def generate_markdown(db):
    md = "# Indice dei Messaggi\n\n"
    # Crea l'indice: per ogni messaggio viene creato un link Obsidian ([[NomeMessaggio]])
    for msg in db.messages:
        md += f"- [[{msg.name}]] (ID: {msg.frame_id}, DLC: {msg.length})\n"
    
    md += "\n---\n\n"

    # Per ogni messaggio crea una sezione dettagliata
    for msg in db.messages:
        md += f"## [[{msg.name}]]\n\n"
        md += f"**ID:** {msg.frame_id} (0x{msg.frame_id:x})\n\n"
        md += f"**DLC:** {msg.length}\n\n"
        if msg.comment:
            md += f"**Commento:** {msg.comment}\n\n"
        md += "### Segnali\n\n"
        if not msg.signals:
            md += "_Nessun segnale definito_\n\n"
        for signal in msg.signals:
            md += f"- **{signal.name}**\n"
            md += f"  - **Start bit:** {signal.start}\n"
            md += f"  - **Lunghezza:** {signal.length}\n"
            md += f"  - **Byte order:** {signal.byte_order}\n"
            md += f"  - **Signed:** {signal.is_signed}\n"
            if signal.unit:
                md += f"  - **Unità:** {signal.unit}\n"
            if signal.minimum is not None and signal.maximum is not None:
                md += f"  - **Range:** [{signal.minimum}, {signal.maximum}]\n"
            if signal.comment:
                md += f"  - **Commento:** {signal.comment}\n"
            # Se sono presenti choices (valori nominali) li aggiunge
            if signal.conversion and hasattr(signal.conversion, "choices") and signal.conversion.choices:
                md += "  - **Choices:**\n"
                for key, val in signal.conversion.choices.items():
                    md += f"    - {key}: {val}\n"
            md += "\n"
        md += "---\n\n"
    return md

def main():
    if len(sys.argv) < 3:
        print("Usage: python generate_markdown.py <input_dbc_file> <output_markdown_file>")
        sys.exit(1)
    
    input_dbc = sys.argv[1]
    output_md = sys.argv[2]
    
    try:
        db = cantools.database.load_file(input_dbc)
    except Exception as e:
        print(f"Errore nel caricamento del file DBC: {e}")
        sys.exit(1)
    
    markdown_content = generate_markdown(db)
    
    with open(output_md, "w", encoding="utf-8") as f:
        f.write(markdown_content)
    
    print(f"File Markdown generato: {output_md}")

if __name__ == "__main__":
    main()
