# 🎯 Voting dApp – Inteligentny kontrakt do głosowań (Solana + Anchor)

Voting dApp to inteligentny kontrakt napisany w języku **Rust** z użyciem frameworka **Anchor**, działający na blockchainie **Solana**.  
Kontrakt umożliwia:

- tworzenie wydarzeń/głosowań,
- dodawanie kandydatów,
- głosowanie przez użytkowników,
- sprawdzanie wyników.

---

## 📦 Funkcjonalności

### ✔️ Tworzenie wydarzenia / głosowania
Osoba upoważniona może utworzyć nowe głosowanie, które zawiera:
- nazwę,
- opis,
- listę kandydatów (początkowo pustą).

### ✔️ Dodawanie kandydatów
Twórca wydarzenia może dodać dowolną liczbę kandydatów.

### ✔️ Oddawanie głosu
Każdy użytkownik może oddać **jeden głos** na wybranego kandydata w danym wydarzeniu.

### ✔️ Sprawdzanie wyników
Każdy może odczytać:
- listę kandydatów,
- liczbę głosów na każdego z nich,
- informacje o wydarzeniu.

---

## ⚙️ Instalacja oprogramowania
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
