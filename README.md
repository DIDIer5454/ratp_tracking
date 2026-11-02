# ratp_tracking

## Fonctionment globale

  Cette application a pour objectif de suivre en temps réel les transport , elle est faite pour Linux.
  Elle suivra les transports de la RATP à partir d'une API et affichera en temps réels le temps d'attente de certaines lignes. 
  On rentrera le début et la fin de trajet lors du lancement du service et le refresh .
  Des fonctionalitées seront à rajouter plus tard en fonction de mon évolution.

## Cahier des charges
L'application **doit** :
  -  Récupérer un trajets préalablement entré.
  -  Récupérer le délais d'attente des stations concernées en temps réel.
  -  Emettre des notifications sur le bureau.

L'application **devrait**:
  - Être lancé en service(daemon).
  - Enregistrer un trajet quotidien
  - Permetre à l'utilisateur de changer de trajet
  - Signaler automatiquement quand l'utilisateur doit se mettre en route via une notification
  - Signaler quand il ya un problème sur la ligne
  - Proposer un itinéraire
    
## Conception logicielle
Plusieurs services :
  - Client API : Il fait les requêtes vers l'API Rest RATP
  - "Moteur" graphique : Il gère l'affichage 
    - Gestionnaire des notifications : Il affiche les notifications , il est géré par le moteur graphique
    - Gestionnaire des fenêtres : Il affiche et gère les fenêtres principales(formulaires et help) ,lui aussi est géré par le moteur graphique.
  - Gestionnaire des services : Il gèrera le lien entre les différents services(commande quoi afficher au Moteur graphique et demande au client de faire des requêtes)
    Ce dernier tournera en daemon et fera des "refresh" d'évènement chaque 5min (ceci peut être modifiable si on a le temps d'implementer cela ).

## Installation 
L'idée est que l'application sera téléchargé à partir du dépot et on lancera le script qui fera l'installe et va créer le service ou sinon on fait un document md expliquant à l'utilisateur quoi faire , dans des versions ultérieurs on transmettra que le binaire avec des instructions.

## Aller plus loin
Pour aller plus loin , on peut:
  -  Revoir l'architecture et séparer les deux sous services d'affichages complètement.
  -  Intégration d'un système interne de comparaison d'itinéraires(pertinence de l'IA à étudier).
  -  (Ouvert à d'autres propositions.)
