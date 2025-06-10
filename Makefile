
# Váriavel opcional contendo o diretório alvo da compilação do módulo
OUT_DIR ?= ./pkg

# Função para limpar a compilação do módulo
clean:
	@echo "Limpando compilação do projeto $(basename $(pwd))"
	@rm -rf pkg
	@rm -rf target
	@rm Cargo.lock

# Função para compilar o módulo
build:
	@echo "Compilando projeto para o target wasm"
	@wasm-pack build --target web --out-dir $(OUT_DIR)