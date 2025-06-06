# Desktop Commander - Guide de Transition Pop!_OS

## Configuration Requise pour Linux

### 1. Changement du Shell par défaut
```bash
# Actuel (Windows)
"defaultShell": "powershell.exe"

# Futur (Pop!_OS)
"defaultShell": "/bin/bash"
```

### 2. Adaptation des Chemins

| Opération | Windows                 | Pop!_OS            |
| --------- | ----------------------- | ------------------ |
| Home      | `C:\\Users\\[username]` | `/home/[username]` |
| Temp      | `%TEMP%`                | `/tmp`             |
| Config    | `%APPDATA%`             | `~/.config`        |
| Separator | `\\`                    | `/`                |

### 3. Commandes Équivalentes

| Tâche        | Windows            | Linux        |
| ------------ | ------------------ | ------------ |
| Lister       | `dir`              | `ls -la`     |
| Lire fichier | `type`             | `cat`        |
| Chercher     | `findstr`          | `grep`       |
| Process      | `tasklist`         | `ps aux`     |
| Kill         | `taskkill /F /PID` | `kill -9`    |
| Path var     | `echo %PATH%`      | `echo $PATH` |

### 4. Permissions Linux

```bash
# Vérifier permissions
ls -l fichier

# Rendre exécutable
chmod +x script.sh

# Changer propriétaire (si nécessaire)
chown user:group fichier
```
### 5. Commandes Bloquées à Revoir

Certaines commandes bloquées sur Windows pourraient être OK sur Linux:
- `mount` (pour monter des drives)
- `dd` (utile pour images disque)
- Mais garder `sudo` bloqué pour sécurité

### 6. Scripts de Test Post-Migration

```bash
#!/bin/bash
# test-desktop-commander.sh

echo "=== Test Desktop Commander on Pop!_OS ==="

# Test 1: File operations
echo "Test 1: Creating test file..."
echo "Hello Pop!_OS" > /tmp/dc-test.txt

# Test 2: Reading file
echo "Test 2: Reading file..."
cat /tmp/dc-test.txt

# Test 3: Directory listing
echo "Test 3: Listing home directory..."
ls -la ~/

# Test 4: Process check
echo "Test 4: Checking processes..."
ps aux | grep claude

# Test 5: System info
echo "Test 5: System information..."
uname -a
lsb_release -a

echo "=== Tests completed ==="
```

### 7. Gotchas Spécifiques Pop!_OS

1. **Case Sensitivity**: Linux est case-sensitive contrairement à Windows
2. **Hidden Files**: Commencent par `.` (utiliser `ls -a`)
3. **Executables**: Pas d'extension `.exe`, vérifier avec `which`
4. **Package Manager**: `apt` au lieu d'installers Windows
### 8. Optimisations Pop!_OS

```bash
# Aliases utiles à ajouter dans ~/.bashrc
alias ll='ls -alF'
alias la='ls -A'
alias l='ls -CF'
alias ..='cd ..'
alias ...='cd ../..'

# Pour Desktop Commander
alias dc-home='cd ~/claude-desktop'
alias dc-logs='tail -f ~/claude-desktop/logs/*'
```

## Checklist Pré-Migration

- [ ] Backup config Desktop Commander actuelle
- [ ] Lister tous les paths hardcodés Windows
- [ ] Préparer mapping commandes Windows→Linux
- [ ] Tester en VM Pop!_OS d'abord
- [ ] Documenter différences de comportement
- [ ] Créer scripts de migration

## Post-Migration

- [ ] Changer defaultShell dans config
- [ ] Tester toutes les opérations de base
- [ ] Ajuster blockedCommands si nécessaire
- [ ] Optimiser pour workflow Linux
- [ ] Documenter nouveaux patterns