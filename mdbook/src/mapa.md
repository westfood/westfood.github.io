# Mapa

Mapa by měl být hexagonální torus, který umožňuje přelít sever do jihu a západ do východu.

## Velikost

- Grid: 100x100 = 10,000 pozic
- 4,000 pozic moře
- 6,000 pozic pro pevninu a řeky
- viewport targets
    - 1920x1080
    - 3840x2160
    - 4096x2160

# Humankind 2560x1080
- nejvyší přiblížení je 12 grids vertikálně
- největší oddálení kdy je vidět krajina - 18 grids, sedm kroků
- osmý krok je náhled
- sedmnáctý krok zmizí detaily měst a jména nepřipojených území
- max zoomout 48 grids, 28 kroků

# Civilization 6 - 2560x1440
- nejvyšší přiblízení je 7 grids vertiálně
- největší oddálení je 20 grids vertikálně

## Implementace
[Hexagonal Grid by Red Blob Games](https://www.redblobgames.com/grids/hexagons/)

## Rozložení krajiny
[Land use](https://ourworldindata.org/land-use)
# Typ pole
Pobřežní voda
Oceán
Travnatá krajina (Grassland)
Travnatá krajin s kopci (Grassland with Hills)
Suchá tráva
Plošina / PLanina (Plains)
Planina s kopci
Planina s horou
Poušť
Flood Plains
Prales
Bažina
Les
Kopec
Hora
Zasněžená Hora
Prérie
Hornaté pole
Hornatý les
Pustina
Neúrodná krajina
Kamené pole
Zalesněná krajina
Oáza
Černozem
Korálový útest
Horké lázně
Clay
Jeskyně
Pramen řeky
Ledovce
Divoký oceán
Led

# Krajina/Biom
Arktická
Badlands
Poušť
Travnatá
Středozemní
Savana
Tajga
Teplá
Tropická
Turnda