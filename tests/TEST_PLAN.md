# Test Organization

## Structure

```
tests/
├── unit/              # Unit tests - funciones individuales
├── integration/       # Integration tests - flujos completos
├── e2e/              # End-to-end tests - con SOPS real
└── helpers/          # Utilidades compartidas
```

## Current Coverage

### ✅ Covered (11 test files)

**Character Handling:**
- `special_chars_test.rs` - Parsing básico
- `comment_handling_test.rs` - Comentarios en ENV/INI
- `quoting_test.rs` - Funciones de entrecomillado
- `json_extraction_test.rs` - Extracción de JSON
- `sops_special_chars_test.rs` - Conversión JSON

**Integration:**
- `integration_quoting_test.rs` - Flujo de entrecomillado
- `agesmith_flow_test.rs` - Flujo completo simulado
- `sops_integration_test.rs` - Integración con SOPS
- `read_test.rs` - Lectura de archivos
- `storage_test.rs` - Almacenamiento

**E2E:**
- `diagnose_sops.sh` - Diagnóstico con SOPS real

## 🎯 Areas to Cover

### 1. Unit Tests (Missing)
- [ ] `config.rs` - Configuración y favoritos
- [ ] `i18n.rs` - Internacionalización
- [ ] `generator.rs` - Generador de secretos
- [ ] `help.rs` - Sistema de ayuda
- [ ] `state.rs` - Lógica de estado (parcial)
- [ ] `ui.rs` - Componentes UI (sin tests)
- [ ] `events.rs` - Manejo de eventos (sin tests)

### 2. Integration Tests (Expand)
- [ ] Flujo completo: crear → editar → guardar → cargar
- [ ] Manejo de errores y recuperación
- [ ] Múltiples formatos (JSON, YAML, ENV, INI)
- [ ] Búsqueda y filtrado
- [ ] Clipboard operations

### 3. Edge Cases (Missing)
- [ ] Archivos vacíos
- [ ] Archivos corruptos
- [ ] Permisos de archivo
- [ ] Archivos muy grandes
- [ ] Caracteres Unicode
- [ ] Rutas con espacios/caracteres especiales

### 4. Security Tests (Missing)
- [ ] Validación de claves age
- [ ] Manejo de claves inválidas
- [ ] Archivos sin encriptar
- [ ] Inyección de comandos

### 5. Performance Tests (Missing)
- [ ] Archivos grandes (1000+ secretos)
- [ ] Búsqueda en archivos grandes
- [ ] Tiempo de encriptación/desencriptación

## 📋 Proposed Test Plan

### Phase 1: Unit Tests (Priority: High)
1. Config management
2. Secret generator
3. I18n translations
4. State mutations

### Phase 2: Integration Tests (Priority: High)
1. Complete CRUD operations
2. Format conversions
3. Error handling
4. Search/filter

### Phase 3: Edge Cases (Priority: Medium)
1. File system edge cases
2. Unicode handling
3. Large files

### Phase 4: Security & Performance (Priority: Low)
1. Security validations
2. Performance benchmarks
