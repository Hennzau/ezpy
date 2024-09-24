# CSPython

CSPython est un outil **alias** ou **mirror** de python qui vise à être utilisé pour ne pas compromettre les variables path de l'utilisateur
simplement comme un alial de Python. C'est à dire que toutes les commandes du type:

```bash
python [args...]
```

peuvent être remplacées par:

```bash
cspython [args...]
```

De plus deux commandes supplémentaires existent pour simplifier certaines procédures:

- `cspython init` qui permet d'initialiser l'environnement python de l'utilisateur, à n'utiliser qu'une seule et unique fois.
- `cspython dir` qui renvoie l'emplacement de l'éxécutable python utilisé par `cspython`, utile pour faire fonctionner `cspython` dans VSCode par exemple.

## Installation

Pour installer `cspython` on dispose de deux scripts, selon si vous êtes sur Windows ou Linux/MacOS. Il faut éxécuter ces scripts dans les invites de commandes disponnible pour chacun de ces OS:

### Linux/MacOS

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Hennzau/zz/releases/download/0.3.1/cspython-installer.sh | sh
```

### Windows

```sh
powershell -ExecutionPolicy ByPass -c "irm https://github.com/Hennzau/zz/releases/download/0.3.1/cspython-installer.ps1 | iex"
```

Ensuite vous devez rentrer la commande

```bash
cspython init
```

## Utilisation

Pour éxécuter un script simple il suffit de faire comme en python, c'est à dire `cspython mon_fichier.py`.  Pour des scripts nécessitants des modules
comme `numpy` ou `matplotlib` il faut d'abord créer un environnement virtuel. Comme avec `python`, on fait:

```bash
cspython -m venv .venv
```

Ensuite, une des fonctionnalités de `cspython` est de trouver si un environnement virtuel a été créé, donc désormais toutes les commandes `cspython [args]` se feront dans l'environnement virtuel. Nul besoin d'activer l'environnement virtuel.

Ainsi ces deux procédures sont équivalentes:

```bash
cspython -m pip install numpy matplotlib
```

```bash
source .venv/bin/activate # On Linux only
pip install numpy matplotlib
```

Enfin, pour rajouter l'interpréteur python `cspython` dans des éditeurs comme VSCode, il est nécessaire de connaître le chemin d'accès à l'interpréteur python.
Cela se fait en utilisant la commande `cspython dir`, qui renverra le python global si aucun environnement virtuel n'a été trouvé, ou le python de l'environnement virtuel si il y'en a un.
