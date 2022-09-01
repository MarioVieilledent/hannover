# PazuLang doc

## Introduction

## Mots clés réservés

|mot clé|signification|
| --- | --- |
|a|array|
|c|constant|
|v|variable|
|e|else|
|f|function|
|f|false|
|f|for|
|i|if|
|n|null|
|p|print function|
|s|scan function|
|t|true|

## Variables

Déclaration

```
v <nom>: <type>;
```

Affectation

```
<nom> = <valeur>;
```

Declaration et affectation

```
v <nom>: <type> = <valeur>
```

## Afficher valeur en console

```
v age: i = 23;
s.print(age);
```

## Types primitifs

|mot clé|type|valeurs possibles|
| --- | --- | --- |
|i|entier signé à 32 ou 64 bits (dépend de l'architecture)|-420, 69, etc.|
|u|entier non signé sur 8 bits|0 à 255|
|c|char codé en unicode sur 2 octets|'a', '6', etc.|
|b|booléen|t ou f|
