#!/usr/bin/env python3
import sys
import os
import cantools
import re

def safe_anchor(text):
    """
    Create a safe anchor for markdown:
    Convert to lowercase and replace non-alphanumeric characters with hyphens.
    """
    anchor = text.lower()
    anchor = re.sub(r'[^a-z0-9]+', '-', anchor)
    return anchor.strip('-')

def generate_index_table(db):
    table = "| Message | ID (dec) | ID (hex) | DLC | Comment |\n"
    table += "| ------- | -------- | -------- | --- | ------- |\n"
    for msg in db.messages:
        comment = msg.comment if msg.comment else ""
        id_dec = f"{msg.frame_id}"
        id_hex = f"0x{msg.frame_id:x}"
        table += f"| [{msg.name}](#{safe_anchor(msg.name)}) | {id_dec} | {id_hex} | {msg.length} | {comment} |\n"
    return table

def generate_id_table(msg):
    table = "| ID (dec) | ID (hex) | DLC |\n"
    table += "| -------- | -------- | --- |\n"
    id_dec = f"{msg.frame_id}"
    id_hex = f"0x{msg.frame_id:x}"
    table += f"| {id_dec} | {id_hex} | {msg.length} |\n\n"
    return table

def generate_signals_table(msg):
    if not msg.signals:
        return "_No signals defined_\n\n"
    
    table = ("| Signal | Start | Length | Byte Order | Signed | Unit | Range | Comment |\n" +
             "| ------ | ----- | ------ | ---------- | ------ | ---- | ----- | ------- |\n")
    for signal in msg.signals:
        range_str = ""
        if signal.minimum is not None and signal.maximum is not None:
            range_str = f"[{signal.minimum}, {signal.maximum}]"
        comment_str = signal.comment if signal.comment else ""
        unit_str = signal.unit if signal.unit else ""
        table += (f"| {signal.name} | {signal.start} | {signal.length} | {signal.byte_order} "
                  f"| {signal.is_signed} | {unit_str} | {range_str} | {comment_str} |\n")
    return table

def generate_value_map_table(msg):
    """
    Create a table for value maps (enumerations) for each signal that defines choices.
    """
    rows = []
    for signal in msg.signals:
        if signal.conversion and hasattr(signal.conversion, "choices") and signal.conversion.choices:
            for key, val in signal.conversion.choices.items():
                rows.append((signal.name, key, val))
    if not rows:
        return ""
    
    table = "| Signal | Key | Value |\n| ------ | --- | ----- |\n"
    for signal_name, key, val in rows:
        table += f"| {signal_name} | {key} | {val} |\n"
    return table

def generate_message_section(msg):
    # Message name as h4 heading
    section = f"#### {msg.name}\n\n"
    
    # Table 1: Message ID details (split in two columns)
    section += generate_id_table(msg)
    
    # Print comment only if present
    if msg.comment:
        section += f"**Comment:** {msg.comment}\n\n"
    
    # Table 2: Signals details
    section += "##### Signals\n\n"
    section += generate_signals_table(msg) + "\n"
    
    # Table 3: Value Maps (if any)
    value_map_table = generate_value_map_table(msg)
    if value_map_table:
        section += "##### Value Maps\n\n"
        section += value_map_table + "\n"
    
    section += "---\n\n"
    return section

def generate_markdown(db):
    md = "# DBC Documentation\n\n"
    md += "## Message Index\n\n"
    md += generate_index_table(db)
    md += "\n---\n\n"
    
    for msg in db.messages:
        md += generate_message_section(msg)
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
        print(f"Error loading DBC file: {e}")
        sys.exit(1)
    
    markdown_content = generate_markdown(db)
    
    output_dir = os.path.dirname(os.path.abspath(output_md))
    if not os.path.isdir(output_dir):
        os.makedirs(output_dir, exist_ok=True)
    
    with open(output_md, "w", encoding="utf-8") as f:
        f.write(markdown_content)
    
    print(f"Markdown file generated: {output_md}")

if __name__ == "__main__":
    main()

