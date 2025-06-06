# Time Server MCP - Installation Guide

## 📅 Decision Context
- **Evaluation Date**: June 2025 (NOT 2024!)
- **Problem Identified**: Claude regularly confuses 2024/2025 dates
- **Solution**: Time MCP as external temporal reference
- **IMPACT Score**: 28/30 - High value integration

## 🔧 Installation Windows (Current)

### Option 1: Via uvx (Recommended)
```bash
# Prerequisites: install uv
pip install uv

# No specific installation needed!
# Claude Desktop configuration only
```

### Option 2: Via pip
```bash
pip install mcp-server-time
```

### Claude Desktop Configuration
1. Open Claude Desktop settings
2. Add to mcpServers:

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

## 🐧 Linux Preparation (Pop!_OS)

### Future Configuration
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

Identical! This is the advantage of MCP - truly cross-platform.

## 📋 Post-Installation Testing

### Test 1: Current Time
```
"What time is it in Marrakech?"
"What time is it in New York?"
```

### Test 2: Time Conversion
```
"If it's 2:30 PM in Marrakech, what time is it in Tokyo?"
```

### Test 3: Date Validation
```
"We are in June 2025, not 2024, correct?"
```

## 📊 Success Metrics

- [ ] No more 2024/2025 errors in responses
- [ ] Instant timezone conversions
- [ ] Precise timestamps in logs
- [ ] Linux transition without changes

## 🔄 Daily Usage Patterns

### Pattern 1: Systematic Validation
Before mentioning any date:
1. Call get_current_time
2. Confirm year/month
3. Respond with confidence

### Pattern 2: International Meetings
```
User: "Schedule a call with US team"
Claude: [check current time + convert] 
"It's currently 3 PM in Marrakech and 9 AM in NY..."
```

### Pattern 3: Logs & Debugging
```
[get_current_time]
"Debug timestamp: 2025-06-05T15:30:00+01:00"
```

## 🚨 Rollback if Necessary

1. Remove from Claude Desktop config
2. `pip uninstall mcp-server-time` (if pip installed)
3. Done!

## 📝 Integration Success Documentation

After successful installation:

```markdown
### MCP Time Server - Temporal Management
**Integrated**: June 2025
**Score IMPACT**: 28/30
**Usage**: Date validation, timezone conversions, timestamps
**Key Commands**: 
- `get_current_time`: Current time in timezone
- `convert_time`: Conversion between timezones
**Gotchas**: Specify --local-timezone=Africa/Casablanca
**ROI**: Eliminates 2024/2025 errors, unified cross-platform
```

## 🎯 Strategic Value

### Problem Solved
- **Temporal confusion**: Claude systematic date validation
- **Timezone management**: Instant conversions for global work
- **Logging precision**: Accurate timestamps in all outputs

### Ecosystem Integration
- **Cross-platform**: Works identically Windows → Linux
- **Zero maintenance**: No ongoing configuration needed
- **Lightweight**: Minimal resource consumption
- **Universal compatibility**: Works with all other MCPs

---

*Installation guide for Agentic Ecosystem Lab MCP integration*
*Updated: 2025-06-06*