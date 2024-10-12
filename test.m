function [x_fixe, iterations] = pointfixeCSMA(x0, N, m, W0, q, tol, max_iter)

% Initialiser les variables
x = x0;            % Valeur initiale pour x
iterations = 0;
erreur = inf;      % Initialiser l'erreur à l'infini

% Itérer jusqu'à ce que le point fixe soit trouvé ou que le nombre maximal d'itérations soit atteint
while erreur > tol && iterations < max_iter
    % Calculer la prochaine valeur de x selon la logique de la fonction
    Pinactif = 1 / (1 + (1 + (W0 - 1) / (2 * (1 - x))) * q * (1 - x^m) / (1 - x));
    tau = q * Pinactif * (1 - x^m) / (1 - x);
    x_next = 1 - (1 - tau)^(N - 1);

    % Calculer l'erreur entre les estimations successives
    erreur = abs(x_next - x);

    % Mettre à jour la valeur actuelle de x
    x = x_next;

    % Incrémenter le compteur d'itérations
    iterations = iterations + 1;
end

% Retourner le point fixe (probabilité de collision) et le nombre d'itérations
x_fixe = x;

end
