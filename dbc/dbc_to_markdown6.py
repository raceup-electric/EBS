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

def generate_index_table(messages):
    table = "| Message | ID (dec) | ID (hex) | DLC | Comment |\n"
    table += "| ------- | -------- | -------- | --- | ------- |\n"
    for msg in messages:
        comment = msg.comment if msg.comment else ""
        id_dec = f"{msg.frame_id}"
        id_hex = f"0x{msg.frame_id:x}"
        # Use native markdown link to the message section using its safe anchor.
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
    section = f"#### {msg.name}\n\n"  # h4 heading for the message name
    # Table 1: Message ID details
    section += generate_id_table(msg)
    
    # Print comment only if exists
    if msg.comment:
        section += f"**Comment:** {msg.comment}\n\n"
    
    # Table 2: Signals details
    section += generate_signals_table(msg) + "\n"
    
    # Table 3: Value Maps (if any)
    value_map_table = generate_value_map_table(msg)
    if value_map_table:
        section += "##### Value Maps\n\n"
        section += value_map_table + "\n"
    
    section += "---\n\n"
    return section

def generate_markdown_for_file(dbc_file):
    try:
        db = cantools.database.load_file(dbc_file)
    except Exception as e:
        return f"Error loading {dbc_file}: {e}\n\n"
    
    content = f"### {os.path.basename(dbc_file)}\n\n"
    # Create an index table for messages in this file
    content += "#### Message Index\n\n"
    content += generate_index_table(db.messages)
    content += "\n---\n\n"
    
    for msg in db.messages:
        content += generate_message_section(msg)
    return content

def generate_markdown(db_files):
    md = "# DBC Documentation\n\n"
    for dbc_file in db_files:
        md += generate_markdown_for_file(dbc_file)
    return md

def main():
    if len(sys.argv) < 3:
        print("Usage: python dbc_to_markdown.py <dbc_file1> <dbc_file2> ... <output_markdown_file>")
        sys.exit(1)
    
    # All arguments except the last are input files.
    input_files = sys.argv[1:-1]
    output_md = sys.argv[-1]
    
    markdown_content = generate_markdown(input_files)
    
    output_dir = os.path.dirname(os.path.abspath(output_md))
    if not os.path.isdir(output_dir):
        os.makedirs(output_dir, exist_ok=True)
    
    with open(output_md, "w", encoding="utf-8") as f:
        f.write(markdown_content)
    
    print(f"Markdown file generated: {output_md}")

if __name__ == "__main__":
    main()
