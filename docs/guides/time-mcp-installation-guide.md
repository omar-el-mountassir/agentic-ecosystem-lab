# Installation MCP Time Server - Guide Pratique

## 📅 Contexte Décisionnel
- **Date évaluation**: Juin 2025 (PAS 2024 !)
- **Problème identifié**: Claude confond régulièrement 2024/2025
- **Solution**: MCP Time comme référence temporelle externe

## 🔧 Installation Windows (Actuel)

### Option 1: Via uvx (Recommandé)
```bash
# Prérequis: installer uv
pip install uv

# Pas d'installation spécifique nécessaire !
# Configuration Claude Desktop uniquement
```

### Option 2: Via pip
```bash
pip install mcp-server-time
```

### Configuration Claude Desktop
1. Ouvrir Claude Desktop settings
2. Ajouter dans mcpServers:

```json
{
  "mcpServers": {
    "time": {
      "command": "uvx",
      "args": ["mcp-server-time", "--local-timezone=Africa/Casablanca"]
    }
  }
}
```

## 🐧 Préparation Linux (Pop!_OS)

### Configuration Future
```json
{
  "mcpServers": {
    "time": {
      "command": "uvx",
      "args": ["mcp-server-time", "--local-timezone=Africa/Casablanca"]
    }
  }
}
```

Identique ! C'est l'avantage du MCP - vraiment cross-platform.
## 📋 Tests Post-Installation

### Test 1: Heure Actuelle
```
"Quelle heure est-il à Marrakech ?"
"Quelle heure est-il à New York ?"
```

### Test 2: Conversion
```
"Si il est 14h30 à Marrakech, quelle heure est-il à Tokyo ?"
```

### Test 3: Validation Date
```
"Nous sommes bien en juin 2025, pas 2024, correct ?"
```

## 📊 Métriques de Succès

- [ ] Plus d'erreurs 2024/2025 dans mes réponses
- [ ] Conversions timezone instantanées
- [ ] Timestamps précis dans les logs
- [ ] Transition Linux sans changement

## 🔄 Usage Quotidien

### Pattern 1: Validation Systématique
Avant de mentionner une date:
1. Appeler get_current_time
2. Confirmer année/mois
3. Répondre avec confiance

### Pattern 2: Meetings Internationaux
```
User: "Schedule un call avec l'équipe US"
Claude: [check current time + convert] 
"Il est actuellement 15h à Marrakech et 9h à NY..."
```

### Pattern 3: Logs & Debugging
```
[get_current_time]
"Debug timestamp: 2025-06-05T15:30:00+01:00"
```
## 🚨 Rollback si Nécessaire

1. Supprimer de Claude Desktop config
2. `pip uninstall mcp-server-time` (si pip)
3. Done !

## 📝 Mise à Jour Préférences

Ajouter après installation réussie:

```markdown
### MCP Time Server - Gestion Temporelle
**Intégré**: Juin 2025
**Score IMPACT**: 28/30
**Usage**: Validation dates, conversions timezone, timestamps
**Commandes clés**: 
- `get_current_time`: Heure actuelle dans timezone
- `convert_time`: Conversion entre timezones
**Gotchas**: Spécifier --local-timezone=Africa/Casablanca
**ROI**: Élimine erreurs 2024/2025, unifie cross-platform
```