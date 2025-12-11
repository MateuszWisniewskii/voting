# 🎯 Voting System – Solana Smart Contracts

![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
![GitHub repo size](https://img.shields.io/github/repo-size/MateuszWisniewskii/voting)
![Solana](https://img.shields.io/badge/Solana-Devnet-linear)
![Anchor](https://img.shields.io/badge/Anchor-Framework-blue)
---

## 📦 Funkcjonalności

### 👑 Voting Manager (Administrator)
* **Tworzenie wydarzeń:** Inicjalizacja nowych ankiet z nazwą, opisem i ramami czasowymi.
* **Obsługa kandydatów:** Dodawanie dowolnej liczby kandydatów w momencie tworzenia ankiety (batch processing).
* **Cross-Program Invocation (CPI):** Bezpieczne wywoływanie kontraktu Voting w celu alokacji kont.

### 🗳️ Voting Client (Użytkownik)
* **Oddawanie głosu:** Każdy użytkownik może oddać **jeden głos** na wybranego kandydata.
* **Walidacja czasu:** Głosowanie możliwe tylko w określonym przedziale czasowym (Unix Timestamp).
* **Transparentność:** Publiczny dostęp do listy kandydatów i bieżących wyników.

---

## 🏗️ Architektura

System działa w oparciu o dwa programy:
1.  **Manager:** Generuje unikalne ID ankiety i poprzez CPI tworzy konta w programie Voting.
2.  **Voting:** Przechowuje stan (liczba głosów) i obsługuje logikę użytkownika końcowego.

```mermaid
graph TD;
    Admin-->|Create Event|Manager;
    Manager-->|CPI: Initialize Poll|Voting;
    Manager-->|CPI: Add Candidates|Voting;
    User-->|Cast Vote|Voting;

---

## ⚙️ Wymagania i Instalacja

Upewnij się, że masz zainstalowane:
- Rust & Cargo
- Solana CLI
- Anchor Version Manager (avm)
- Node.js & Yarn

[Wszystkie wymagane kroki konfiguracji znajdują się w oficjalnej dokumentacji Anchor](https://www.anchor-lang.com/docs/installation)

---

## 🔑 Konfiguracja dostępu do repozytorium (SSH)
### Sprawdź, czy masz już klucz SSH
``` bash
ls ~/.ssh
```

Jeśli widzisz pliki typu id_ed25519 i id_ed25519.pub, możesz je wykorzystać i pominąć nastepny krok.

### Jeśli nie masz klucza — wygeneruj nowy
``` bash
ssh-keygen -t ed25519 -C "twoj_email@example.com"
```
Po prostu akceptuj domyślne lokalizacje, naciskając ENTER.

### Dodaj klucz na GitHubie
Wyświetl klucz publiczny:
``` bash
cat ~/.ssh/id_ed25519.pub
```
Skopiuj wygenerowany ciąg znaków i wklej go w:

[GitHub -> SSH and GPG keys](https://github.com/settings/ssh/new)


### Ustaw używanie SSH do repo
Jeśli klonujesz repo:
``` bash
git clone git@github.com:MateuszWisniewskii/voting.git
```
Jeśli chcesz przełączyć z HTTPS na SSH:
``` bash
git remote set-url origin git@github.com:UŻYTKOWNIK/REPO.git
```
---
## 🛠️ Budowanie projektu
W folderze projektu:
``` bash
anchor build
```
## 🧪 Testowanie projektu
Uruchom testy integracyjne:
``` bash
anchor test
```
W razie problemów z testami upewnij się, że dodane są:
- anchor-bankrun
- solana-bankrun

Można je dodać za pomocą komendy:
``` bash
yarn add anchor-bankrun
```
oraz:
``` bash
yarn add solana-bankrun
```
Więcej informacji na temat tych bibliotek można znaleźć w ich repozytoriach:
- [repozytorium anchor-bankrun](https://github.com/kevinheavey/anchor-bankrun)
- [repozytorium solana-bankrun](https://github.com/kevinheavey/solana-bankrun) / [dokumentacja](https://kevinheavey.github.io/solana-bankrun/)

