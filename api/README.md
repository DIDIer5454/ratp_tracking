# API

## Description 

Le but de ce module sera la gestion de la récupération des données vers le site de la RATP.

``Client_API`` gèrera la connection à l'api de la ratp et se chargera de commander la construction des objets nécessaires afin de ne pas avoir à faire des requêtes constaments.
L'idée est vraiment qu'il ne s'occupe que des requetes et les transmettent au service qui sera en charge de l'afficher, la création des objets est là pour que éviter de multiple requete (l'idée de création d'un fichier est justement séduisante en ce sens).

example de fichier config utilisé pour les test:

```
url:"https://geo.api.gouv.fr/communes?codePostal=78000" 
api_key:"key"
```
