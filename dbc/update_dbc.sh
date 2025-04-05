bin/bash
# Script per aggiornare il file Markdown con i dettagli del file DBC
# Assicurati che:
# - Il file DBC (qui "can.dbc") esista nella directory corrente (oppure specifica il percorso corretto)
# - Lo script Python generate_markdown.py sia presente nella stessa cartella o modifica il percorso

# Percorso del file DBC
DBC_FILE="can2.dbc"
# Percorso del file Markdown di output
OUTPUT_FILE="$HOME/Obsidian/3.Raceup/DBC.md"

# Controlla se il file DBC esiste
if [ ! -f "$DBC_FILE" ]; then
    echo "Errore: il file DBC '$DBC_FILE' non esiste."
    exit 1
fi

# Esegui lo script Python per generare il file Markdown
python3 dbc_to_markdown.py "$DBC_FILE" "$OUTPUT_FILE"

if [ $? -eq 0 ]; then
    echo "File Markdown aggiornato correttamente: $OUTPUT_FILE"
else
    echo "Si è verificato un errore durante l'aggiornamento del file Markdown."
    exit 1
f
