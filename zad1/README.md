# Raport z zadania 1

## Metodyka testowa

Potrzebne dane były generowane na miejscu za pomocą `std::mt19937`.

Sprzęt:

-   CPU: Ryzen 5 3600 3.6GHz

## Opis badanych operacji

### Tablica

Tablica to kolekcja przechowująca elementy w ciągłym bloku pamięci w sposób uporządkowany, jeden za drugim (+padding).
Ponieważ zadanie wymaga aby struktura rosła by zapewnić miejsce większej ilości elementów niż początkowo wynosiła jej
pojemność, implementowana struktura jest raczej bliższa `std::vector` niż `std::array`.

#### Tworzenie

Utworzenie pustej tablicy a następnie wpisywanie kolejnych `n` elementów.

#### Wstawianie

Wstawianie elementów na arbitralnych pozycjach częściowo lub w pełni wypełnionej tablicy. Przesuwamy elementy od danego
indeksu włącznie do ostatniego elementu o jedną pozycję w prawo, a następnie na zwolnionym miejscu zapisujemy nowy
element. W przypadku częściowego wypełnienia tablicy, wstawiamy do momentu zapełnienia, po czym realokujemy tablicę tak,
by jej nowa pojemność wynosiła `2 * poprzednia_pojemnosc` po czym opcjonalnie zaokrąglamy do jakiejś wielokrotności
którejś potęgi dwójki żeby upewnić się że nowa pojemność jest pełną wielokrotnością wielkości linii cache.

#### Dodawanie

To samo co wyżej, z różnicą że wstawiamy zawsze na koniec tablicy, czyli nie musimy przesuwać istniejących elementów.

#### Wyszukiwanie

Proste przejście tablicy i zwrócenie indeksu dla danego elementu, jeżeli występuje w tablicy.

#### Usuwanie

Zmniejszenie rozmiaru tablicy jeżeli chcemy usunąć ostani element, w przeciwnym wypadku uprzednio przesuwamy elementy od
danego indeksu wyłącznie o jedną pozycję w lewo.

### Lista

Podwójnie linkowana lista przechowuje swoje elementy jako zbiór pojedynczo zaalokowanych bloków połączonych ze sobą za
pomocą wskaźników. Lista przechowuje wskaźniki do pierwszego oraz ostatniego elementu, a bloki zawierają wskaźniki na
blok poprzedni oraz następny. W zależności od wielkości typu przechowywanego, ta struktura może narzucać spory overhead
pamięci (w ekstremalnym przypadku np. podwójnie linkowanej listy `bool`i lub `char`ów, element waży 1 bajt, a para
wskaźników waży 16 bajtów, dając łączny rozmiar bloku równy 17 bajtów, gdzie przechowywany element to zaledwie ~6%
rozmiaru bloku) a także nie jest przyjazna pamięci cache (trawersja tej struktury wymaga skoków w możliwie dalekie od
siebie obszary pamięci). W przeciwieństwie do tablicy, zapewnia jednak O(1) dla operacji wstawiania i usuwania (jeżeli
mamy bezpośrednią referencję do danego bloku).

Mierzenie wydajności tej struktury jest o tyle wymagające, że zazwyczaj przypadki jej użycia są mało trywialne i
zawierają wiele kroków (wyszukiwanie elementu, wstawianie elementów w jego sąsiedztwie, usunięcie elementów do których
mamy już referencję, etc.), więc przypadki testowe dobrze pokrywające przypadki użycia muszą być bardzo duże i
skomplikowane. Mimo tego, wydajność listy będzie mierzona w podobnie trywialny sposób jak w wypadku tablicy.

#### Tworzenie

Tworzenie listy o wielkości `n` osiągnięte jest poprzez utworzenie pustej listy, a następnie zamienne wywoływanie metod
`push_back` oraz `push_front`, aby zweryfikować poprawność działania wstawiania zarówno na początku listy, jak i na jej
końcu.

#### Wstawianie

Wstawianie elementów na arbitralnych indeksach listy wymaga najpierw jej trawersji aż do interesującego nas indeksu,
zatem pomimo O(1) operacji wstawiania, w tym wypadku złożnoność będzie równa złożoności dostępu, czyli O(n).

#### Dodawanie

Dodawanie nowych elementów na koniec lub początek listy wymaga tylko alokacji nowego bloku oraz czterokrotnego
przypisania wartości potrzebnych wskaźników.

#### Wyszukiwanie

Tak jak w wypadku tablicy, wykonujemy trawersję struktury w poszukiwaniu elementu, z tą różnicą że zamiast zwracać
indeks, zwracamy referencję do bloku aby umożliwić szybkie usunięcie elementu lub dodania nowych elementów w jego
sąsiedztwie.

#### Usuwanie

Jak w wypadku wstawiania, bezpośrednie usunięcie bloku ma złożoność O(1), natomiast wstawianie używając indeksu ma
złożnoność O(n), ponieważ wymaga trawersji listy.

### Stos

#### Tworzenie

#### Wstawianie

#### Dodawanie

#### Wyszukiwanie

#### Usuwanie

### Kolejka

#### Tworzenie

#### Wstawianie

#### Dodawanie

#### Wyszukiwanie

#### Usuwanie

## Źródła

-   kod źródłowy:
    [https://github.com/Bravo555/data-structures-project](https://github.com/Bravo555/data-structures-project)
