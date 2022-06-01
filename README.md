# brave-new-rust
Repo to learn about Rust and play a little with it

___
## Passo a passo que fiz pra instalar as ferramentas e debugar

- instalei Visual Studio Build Tools
- Instalei Rust
- Instalei Git
- Instalei GitHub CLI
- Instalei extension "Rust for Visual Studio Code" no VSCode
- Instalei extension "CodeLLDB" no VSCode
- Instalei extensions do github no VSCode

- Depois, num terminal com cmd:
```
git config --global user.name "Fulano de Tal"
git config --global user.email fulanodetal@exemplo.br
gh auth login
gh repo clone amadeuvf/brave-new-rust
```

- Na mesma pasta, criei o "projeto" pelo cargo (usei init pq o new não pode ser numa pasta pre existente e eu fiz o clone antes)
```
cargo init brave-new-rust
```
- Pra finalizar criei algumas configurações automaticas pra dar build e run no hello world.

Resultado:

![image](https://user-images.githubusercontent.com/30466304/170170763-a1a39f32-1d7f-41a7-99b1-ed56cb27c0f2.png)
