# Arquitectura TUI-SOPS

## Diagrama de Módulos

```
┌─────────────────────────────────────────────────────────────┐
│                         main.rs                              │
│                    (Entry Point - 94 LOC)                    │
│  • Inicialización                                            │
│  • Loop de eventos                                           │
│  • Gestión de terminal                                       │
└────────┬────────────────────────────────────────────────────┘
         │
         ├──────────────┬──────────────┬──────────────┐
         │              │              │              │
         ▼              ▼              ▼              ▼
┌────────────┐  ┌────────────┐  ┌────────────┐  ┌────────────┐
│ config.rs  │  │  sops.rs   │  │ state.rs   │  │  events.rs │
│  (60 LOC)  │  │ (175 LOC)  │  │ (564 LOC)  │  │ (429 LOC)  │
├────────────┤  ├────────────┤  ├────────────┤  ├────────────┤
│ • Config   │  │ • AgeKey   │  │ • App      │  │ • Handlers │
│ • Load     │  │ • Decrypt  │  │ • InputMode│  │ • KeyEvent │
│ • Save     │  │ • Encrypt  │  │ • Business │  │ • Routing  │
│ • Favs     │  │ • Keys     │  │   Logic    │  │            │
└────────────┘  └────────────┘  └──────┬─────┘  └────────────┘
                                       │
                        ┌──────────────┼──────────────┐
                        │              │              │
                        ▼              ▼              ▼
                ┌────────────┐  ┌────────────┐  ┌────────────┐
                │   ui.rs    │  │generator.rs│  │  help.rs   │
                │ (571 LOC)  │  │  (57 LOC)  │  │  (33 LOC)  │
                ├────────────┤  ├────────────┤  ├────────────┤
                │ • Render   │  │ • Password │  │ • Commands │
                │ • Panels   │  │ • Tokens   │  │ • Shortcuts│
                │ • Modals   │  │ • UUID     │  │            │
                │ • Layout   │  │            │  │            │
                └────────────┘  └────────────┘  └────────────┘
```

## Flujo de Datos

```
┌──────────┐
│  Usuario │
└────┬─────┘
     │ Tecla
     ▼
┌─────────────┐
│  events.rs  │ ◄─── Enruta eventos por InputMode
└──────┬──────┘
       │ Acción
       ▼
┌─────────────┐
│  state.rs   │ ◄─── Modifica estado de la app
└──────┬──────┘
       │ Llama
       ▼
┌─────────────┐
│  sops.rs    │ ◄─── Operaciones de encriptación
│ config.rs   │ ◄─── Carga/guarda configuración
│generator.rs │ ◄─── Genera secretos
└──────┬──────┘
       │ Resultado
       ▼
┌─────────────┐
│  state.rs   │ ◄─── Actualiza estado
└──────┬──────┘
       │ Estado
       ▼
┌─────────────┐
│   ui.rs     │ ◄─── Renderiza interfaz
└──────┬──────┘
       │ Frame
       ▼
┌─────────────┐
│  Terminal   │
└─────────────┘
```

## Responsabilidades por Capa

### 🎯 Presentación (UI Layer)
- **ui.rs**: Renderizado de componentes visuales
- **help.rs**: Contenido estático de ayuda

### 🎮 Control (Event Layer)
- **events.rs**: Manejo de entrada del usuario
- **main.rs**: Coordinación del loop principal

### 💼 Lógica de Negocio (Business Layer)
- **state.rs**: Estado de la aplicación y operaciones
- **generator.rs**: Generación de secretos

### 🗄️ Datos (Data Layer)
- **sops.rs**: Operaciones de encriptación/desencriptación
- **config.rs**: Persistencia de configuración

## Patrones de Diseño Aplicados

### 1. **Separation of Concerns**
Cada módulo tiene una responsabilidad única y bien definida.

### 2. **Model-View-Controller (MVC)**
- **Model**: state.rs, sops.rs, config.rs
- **View**: ui.rs, help.rs
- **Controller**: events.rs, main.rs

### 3. **Command Pattern**
events.rs encapsula acciones del usuario como comandos.

### 4. **State Pattern**
InputMode enum define diferentes comportamientos según el estado.

### 5. **Facade Pattern**
sops.rs proporciona una interfaz simplificada para operaciones complejas.

## Dependencias entre Módulos

```
main.rs
  ├─→ config.rs
  ├─→ sops.rs
  ├─→ state.rs
  │    ├─→ config.rs
  │    ├─→ sops.rs
  │    └─→ generator.rs
  ├─→ events.rs
  │    ├─→ state.rs
  │    └─→ sops.rs
  └─→ ui.rs
       ├─→ state.rs
       └─→ help.rs
```

## Métricas de Calidad

| Métrica | Antes | Después | Mejora |
|---------|-------|---------|--------|
| Líneas en main.rs | 1,959 | 94 | 95% ⬇️ |
| Módulos | 2 | 8 | 4x 📈 |
| Warnings clippy | 5 | 0 | 100% ✅ |
| Cohesión | Baja | Alta | ⭐⭐⭐ |
| Acoplamiento | Alto | Bajo | ⭐⭐⭐ |

## Ventajas de la Nueva Arquitectura

1. **Mantenibilidad**: Cambios localizados en módulos específicos
2. **Testabilidad**: Módulos independientes fáciles de testear
3. **Escalabilidad**: Fácil agregar nuevas funcionalidades
4. **Legibilidad**: Código organizado y autodocumentado
5. **Reutilización**: Componentes modulares reutilizables
