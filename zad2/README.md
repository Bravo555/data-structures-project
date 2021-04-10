# Zadanie 2 - sortowanie (draft)

Realizacja następnego zadania powinna mieć następujące poprawki względem poprzedniego zadania:

-   co najmniej dwie implementacje problemu w różnych językach programowania celem porównania (w tym przypadku niech
    będzie to C++ i Rust)
-   wgląd w wygenerowany kod asemblerowy, zbadanie autowektoryzacji kodu, zbliżenie się do limitu wydajności sprzętu
-   zbadanie szczegółów używanych zegarów, ich precyzji, rozdzielczości, mikrobenchmarki
-   większe skupienie na wpływie architektury CPU na badany algorytm: cache, branch prediction, etc.
-   większa rygorystyka pomiaru pamięci, badanie zużycia w wielu punktach działania programu i wyświetlanie zużycia
    pamięci over time, używając narzędzi typu `perf`
-   jakieś ładne GUI

## Sformułowanie zadania

Zadanie polega na zaimplementowaniu kilku algorytmów sortowania oraz określeniu ich złożoności czasowej i pamięciowej.
Testowanymi algorytmami będą quicksort w wersji rekurencyjnej oraz radix sort.

## Opis algorytmów

### Quicksort

Quicksort (sortowanie szybkie) jest algorytmem sortowania w miejscu. Polega na rekursywnym wybieraniu spośród kluczy
elementu rozdzielającego (pivota), a następnie dzielenie reszty kluczy na dwie pod-tablice, odpowiednio mniejsze i
większe od pivota.

Pseudokod:

```
algorithm quicksort(A, lo, hi) is
    if lo < hi then
        p := partition(A, lo, hi)
        quicksort(A, lo, p - 1)
        quicksort(A, p + 1, hi)

algorithm partition(A, lo, hi) is
    pivot := A[hi]
    i := lo
    for j := lo to hi do
        if A[j] < pivot then
            swap A[i] with A[j]
            i := i + 1
    swap A[i] with A[hi]
    return i
```

### Radix sort

Radix sort (sortowanie pozycyjne) jest rodzajem sortowania które polega na sortowaniu kluczy kolejno względem wartości
ich kolejnych cyfr w danym systemie liczbowym. Sortujemy od najmniej znaczącej cyfry, do najbardziej znaczącej. Do
kolejnych rund sortowania używamy najczęściej algorytmu counting sort (sortowanie przez zliczanie).

Algorytm ten jest bardzo popularny ponieważ jest czymś pomiędzy bucket sort a algorytmami sortowania in-place. Wadą
bucket sort jest to, że musimy znać zakres wartości naszych danych aby dobrze określić ilość i zakres koszy do
sortowania, co wpływa na zużycie pamięci algorytmu. Dzięki parokrotnemu sortowaniu kolejnych cyfr, eliminujemy tą
potrzebę, zakres wartości jest określony przez bazę wybranego systeu liczbowego. Ponadto, odpowiednio dobierając bazę,
mamy bardzo granularną kontrolę nad ilością pamięci wykorzystywaną przez nasz algorytm, co może bardzo przyśpieszyć
sortowanie względem algorytmów o stałym zużyciu pamięci.

## Dane wejściowe i wyjściowe

Dane wejściowe generowane są przez generator liczb pseudolosowych zainicjalizowany stałym ziarnem, co zapewni
niezmienność danych pomiędzy kolejnymi uruchomieniami programu. Po uruchomieniu programu i przetworzeniu pliku
konfiguracyjnego, program kolejno będzie generował unikalne instancje względem rozmiaru (np. jeżeli w pliku
konfiguracyjnym mamy dwie instancje o takim samym rozmiarze, to program wygeneruje jedną instancję o takim rozmiarze) do
bufora read-only. Przy wykonywaniu przypadków testowych, wygenerowane instancje będą kopiowane do bufora roboczego na
potrzeby wykonania sortowania.
