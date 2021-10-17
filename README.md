# Rust QR code Decoder
Decodificador de códigos QR feito durante recesso de 1 semana da universidade.

### Funcionamento
A **entrada** é uma sequência de 1 e 0 que representa os pontos pretos e brancos de um código QR.

A **saída** é a exibição na tela do conteúdo armazenado nele.

Observações:
- Só foi testado com códigos da versão 3 (tamanho 29 x 29)
- Só é capaz de entender códigos que usam a codificação de byte (não decodifica códigos alfanuméricos, numéricos, etc.)

### Exemplo de execução
Nesta screenshot é possível ver o final de uma execução, com códigos desenhados na tela para melhor entendimento do
funcionamento do algoritmo e com o conteúdo decodificado escrito em seguida.

![Screenshot de uma execução mostrando uma parte de um código QR e um outro por inteiro, desenhados no terminal com marcações. Ao final está escrito "Tamanho: 47 Conteúdo: Aparentemente o algoritmo funciona corretamente"](Execution_Screenshot.png)