
# Ideas sobre comporamiento de enemigos:

1. **Modelar Comportamiento con Máquinas de Estados**: Utiliza una máquina de estados para definir los posibles comportamientos de los enemigos (defensivo, ofensivo, neutro, etc.).

2. **Acciones Basadas en el Jugador**: Haz que la IA reaccione a las últimas acciones del jugador. Por ejemplo, si el jugador ha estado atacando agresivamente, el enemigo podría adoptar una postura más defensiva.

3. **Comportamientos Específicos de Enemigos**: Diferentes tipos de enemigos podrían tener estrategias únicas. Por ejemplo, un enemigo más rápido podría enfocarse en esquivar y contraatacar, mientras que uno más fuerte podría ser más agresivo.

# Tipologías de enemigos

1. **Enemigo Ágil y Escurridizo**: Este enemigo podría tener un estado que favorezca la esquiva y el contraataque. Podría ser especialmente sensible a patrones de ataques repetitivos por parte del jugador, adaptándose rápidamente para evitar daño.

2. **Enemigo Pesado y Fuerte**: Este tipo de enemigo podría tener estados que favorezcan la agresividad y la resistencia. Podría ignorar ataques más débiles y centrarse en acercarse al jugador para infligir daño masivo.

3. **Estados Personalizados**: Aparte de estados genéricos como "ofensivo" o "defensivo", podrías añadir estados que solo ciertos tipos de enemigos pueden usar. Por ejemplo, un enemigo con habilidades de curación podría tener un estado "Curativo" que activa cuando su salud baja a un cierto punto.
