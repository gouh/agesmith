#!/bin/bash
# Script para diagnosticar problemas con SOPS y caracteres especiales

set -e

echo "=== Diagnóstico de SOPS con caracteres especiales ==="
echo

# Verificar que SOPS está instalado
if ! command -v sops &> /dev/null; then
    echo "❌ SOPS no está instalado"
    exit 1
fi

echo "✅ SOPS version:"
sops --version
echo

# Crear directorio temporal
TEST_DIR=$(mktemp -d)
echo "📁 Directorio de prueba: $TEST_DIR"
cd "$TEST_DIR"

# Verificar si hay claves age configuradas
if [ ! -f ~/.config/sops/age/keys.txt ]; then
    echo "⚠️  No hay claves age en ~/.config/sops/age/keys.txt"
    echo "   Creando clave de prueba..."
    mkdir -p ~/.config/sops/age
    # Nota: Necesitarías age instalado para esto
    if command -v age-keygen &> /dev/null; then
        age-keygen -o ~/.config/sops/age/keys.txt 2>/dev/null || true
    fi
fi

# Obtener la primera clave pública
if [ -f ~/.config/sops/age/keys.txt ]; then
    AGE_PUBLIC_KEY=$(grep "# public key:" ~/.config/sops/age/keys.txt | head -1 | awk '{print $4}')
    echo "🔑 Clave age pública: $AGE_PUBLIC_KEY"
    echo
fi

# Test 1: JSON con caracteres especiales
echo "=== Test 1: JSON con caracteres especiales ==="
cat > test.json <<EOF
{
  "simple": "password123",
  "with_equals": "pass=word=123",
  "with_hash": "pass#word#123",
  "with_semicolon": "pass;word;123",
  "with_quotes": "pass\\"word\\"123",
  "complex": "P@\$\$w0rd!#%&*()=+[]{}|;:'\",.<>?/\\\\~\`"
}
EOF

echo "Contenido original:"
cat test.json
echo

if [ -n "$AGE_PUBLIC_KEY" ]; then
    echo "Encriptando con SOPS..."
    sops --encrypt --age "$AGE_PUBLIC_KEY" test.json > test.enc.json 2>/dev/null || {
        echo "⚠️  No se pudo encriptar (puede que necesites configurar .sops.yaml)"
    }
    
    if [ -f test.enc.json ]; then
        echo "✅ Archivo encriptado creado"
        echo
        
        echo "Desencriptando..."
        sops --decrypt test.enc.json > test.dec.json
        
        echo "Contenido desencriptado:"
        cat test.dec.json
        echo
        
        echo "Comparando valores..."
        diff test.json test.dec.json && echo "✅ JSON: Valores idénticos" || echo "❌ JSON: Valores diferentes"
        echo
    fi
fi

# Test 2: Convertir JSON a dotenv
echo "=== Test 2: Conversión JSON -> dotenv ==="
cat > test_env.json <<EOF
{
  "SIMPLE": "password123",
  "WITH_EQUALS": "pass=word=123",
  "WITH_HASH": "pass#word#123",
  "WITH_SEMICOLON": "pass;word;123"
}
EOF

if [ -n "$AGE_PUBLIC_KEY" ]; then
    echo "Encriptando y convirtiendo a dotenv..."
    sops --encrypt --age "$AGE_PUBLIC_KEY" --input-type json --output-type dotenv test_env.json > test.env 2>/dev/null || {
        echo "⚠️  No se pudo convertir a dotenv"
    }
    
    if [ -f test.env ]; then
        echo "Contenido .env encriptado:"
        cat test.env
        echo
        
        echo "Desencriptando .env..."
        sops --decrypt test.env > test.dec.env
        
        echo "Contenido .env desencriptado:"
        cat test.dec.env
        echo
        
        echo "Verificando valores:"
        while IFS='=' read -r key value; do
            [ -z "$key" ] && continue
            [[ "$key" =~ ^#.* ]] && continue
            echo "  $key = '$value'"
            
            # Verificar caracteres problemáticos
            if [[ "$value" == *"#"* ]]; then
                echo "    ⚠️  Contiene #"
            fi
            if [[ "$value" == *"="* ]]; then
                echo "    ⚠️  Contiene ="
            fi
        done < test.dec.env
        echo
    fi
fi

# Test 3: Convertir JSON a INI
echo "=== Test 3: Conversión JSON -> INI ==="
cat > test_ini.json <<EOF
{
  "DEFAULT": {
    "simple": "password123",
    "with_equals": "pass=word=123",
    "with_hash": "pass#word#123",
    "with_semicolon": "pass;word;123"
  }
}
EOF

if [ -n "$AGE_PUBLIC_KEY" ]; then
    echo "Encriptando y convirtiendo a INI..."
    sops --encrypt --age "$AGE_PUBLIC_KEY" --input-type json --output-type ini test_ini.json > test.ini 2>/dev/null || {
        echo "⚠️  No se pudo convertir a INI"
    }
    
    if [ -f test.ini ]; then
        echo "Contenido .ini encriptado:"
        cat test.ini
        echo
        
        echo "Desencriptando .ini..."
        sops --decrypt test.ini > test.dec.ini
        
        echo "Contenido .ini desencriptado:"
        cat test.dec.ini
        echo
    fi
fi

echo "=== Limpieza ==="
cd -
echo "Archivos de prueba en: $TEST_DIR"
echo "Para limpiar: rm -rf $TEST_DIR"
