# Raytracer

Ray tracer desenvolvido como projeto de avaliação para a disciplina de Computação Gráfica I.

# Componentes

Giovanna Batista e Pedro Vinícius.

# Compilação

Com [rust](https://rustup.rs) instalado, execute o comando a seguir para compilar e executar o ray tracer:
```
cargo run --release -- <arquivo_de_cena>
```

# Funcionalidades

- Geração de imagem nos formatos PPM e PNG.
- Plano de fundo com gradiente ou cor sólida.
- Câmeras ortográfica e de perspectiva.
- Integradores `flat`, `normal_map`, `blinn_phong` e `cel_shading`.
- Luzes de tipos `ambient`, `point`, `directional` e `spotlight`.
- Materiais `flat`, `blinn`, `cel` e `checkerboard`.
- Suporte a esferas, planos infinitos e malhas triangulares.
- Leitura de malhas de arquivos `.obj`.
- Acelerador de renderização com `BVH`.
- Transformações geométricas.

# Extras

- Integrador `normal_map`.
- Atenuação de luz.
- Efeitos de dithering com `bayer`, `white_noise` e `blue_noise`.

Também há um gerador de cena com uma pirâmide de esferas, que pode ser executado com:
```
cargo run --release --bin sphere_pyramid -- <altura_da_piramide>
```
Isso irá gerar o arquivo `scenes/scene_pyramid.xml`.
