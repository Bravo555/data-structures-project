# Zadanie 3

Badanie implementacji struktur grafowych - objętości pamięciowej oraz złożoności obliczeniowej algorytmu Dijkstry.

## Badane implementacje

### Macierz sąsiedztwa

Graf reprezentowany jest w postaci macierzy n x n, gdzie n to liczba wierzchołków. Połączenie z wierzchołka a i b jest
zapisywane jako `macierz_{ab} = waga`.

\[ jakis obrazek tutaj \]

### Lista sąsiedztwa

Graf reprezentowany jest w postaci listy trójek `(wierzcholek_start, wierzcholek_koniec, waga)`.

\[ jakis obrazek tutaj \]

## Badany algorytm najkrótszej ścieżki Dijkstry

## Procedura badawcza

Mierzone były czas wykonania oraz zużycie pamięci. Wykonane zostały pomiary dla następujących operacji:

-   znajdywanie najkrótszych ścieżek algorytmem Dijkstry w grafie bez ujemnych wag
-   znajdywanie najkrótszych ścieżek algorytmem Dijkstry w grafie z ujemnymi wagami

Dane wejściowe generowane są przez generator liczb pseudolosowych zainicjalizowany stałym ziarnem, co zapewni
niezmienność danych pomiędzy kolejnymi uruchomieniami programu. Po uruchomieniu programu i przetworzeniu pliku
konfiguracyjnego, program kolejno będzie generował instancje o rozmiarze podanym w pliku. Następnie na tych instancjach
będą wykonywane operacje wstawiania, wyszukiwania oraz usuwania.

## Wyniki i analiza
