# zz

`zz` permet de s'affranchir des difficultés rencontrés avec des étudiants lors des installations python (multiple installations, conflits entre des paquets...)

Avec `zz`, une seule version python est installée à un endroit précis de la machine, pour pouvoir y accéder sans modifier les variables d'environnements de l'utilisateur.

Vous pouvez donc lancer un script python en utilisant la commande `zz` suivie du nom du script python (avec de potentiels arguments).
Mais si vous décidez d'installer des packages supplémentaires vous devez le faire avec la commande `zz install`, cela crééra automatiquemenet un venv, le liera au Code Editor ouvert.
Vous pouvez aussi utiliser `zz dir` qui vous donnera le path vers l'éxécutable python à utiliser (pratique pour le set manuellement dans VSCode).
