{
    // --- Aparência e Interface ---
    "workbench.colorTheme": "Default Dark Modern",
    "editor.bracketPairColorization.enabled": true,
    "editor.guides.bracketPairs": "active",
    "editor.minimap.enabled": false, 
    "workbench.sideBar.location": "left",
    "editor.renderWhitespace": "none",
    "workbench.startupEditor": "none",

    // --- Editor e Escrita ---
    "editor.fontSize": 14,
    "editor.lineHeight": 22,
    "editor.fontLigatures": false, 
    "editor.cursorBlinking": "smooth",
    "editor.smoothScrolling": true,

    // --- Indentação e Salvamento ---
    "editor.tabSize": 4,
    "editor.insertSpaces": false, 
    "files.autoSave": "afterDelay",
    "files.autoSaveDelay": 1000, 
    "editor.formatOnSave": false, 

    // --- Terminal ---
    "terminal.integrated.fontSize": 14,
    "terminal.integrated.copyOnSelection": true,

    // --- Configurações Específicas por Linguagem ---
    "[python]": {
        "editor.defaultFormatter": "ms-python.python",
        "editor.formatOnType": true
        "python.analysis.autoImportCompletions": true,
        "python.analysis.typeCheckingMode": "basic"
    },
    "[c]": {
        "editor.wordBasedSuggestions": "matchingDocuments"
    },
    "[cpp]": {
        "editor.wordBasedSuggestions": "matchingDocuments"
    },
    "[rust]": {
        "editor.semanticHighlighting.enabled": true
    },
    "[markdown]": {
        "editor.wordWrap": "on",
        "editor.quickSuggestions": {
            "comments": "off",
            "strings": "off",
            "other": "off"
        }
    },

    "editor.rulers": [80, 120], 

    "extensions.ignoreRecommendations": true,
    "explorer.confirmDragAndDrop": false,
    "terminal.integrated.enableMultiLinePasteWarning": "never",
    "workbench.secondarySideBar.defaultVisibility": "hidden",

}