# Size of population grid
ROWS = 50
COLS = 20

# Number of generations to run through
NUM_GENS = 200

# True/False threshold for population init
SOME = 0.619

# Adds int conversion to false
class FalseClass
  def to_i
    0
  end
end

# Adds int conversion for true
class TrueClass
  def to_i
    1
  end
end

# Clears terminal and starts creation of population array
def start
  sleep( 1 )
  clear_screen
  life( ROWS, COLS, SOME, NUM_GENS )
end

# Creates array of people and calls recursive func to advance gens
def life ( rows, cols, some, gens )
  # Array to hold people
  population = Array.new( rows * cols )
  # Random float generator
  floatgen = Random.new
  # Init population with random bools
  0.upto( population.length - 1 ) do |i|
    population[i] = floatgen.rand( max = 1.0 ) < some
  end
  live( population, rows, cols, gens )
end

# Recursive function to advance generations of population
def live ( population, rows, cols, gens )
  # Base case
  if gens < 1
    return 0
  end
  # Reset screen for new gen
  sleep( 0.1 )
  clear_screen
  # Print status of current gen
  printf( "Generation %3d\n", NUM_GENS - gens )
  count = 0
  population.each do |person|
    printf( person ? "o" : " " )
    count += 1
    if count % cols == 0
      printf("\n")
    end
  end
  # Update population for next gen
  next_gen = new Array(population.length)
  0.upto(population.length - 1) do |i|
    # Check number of living neighbors
    neighbors = [population[i - 1], population[i + 1], population[i - cols - 1],
                 population[i - cols], population[i - cols + 1],
                 population[i + cols - 1], population[i + cols], population[ i + cols + 1]]
    live_neighbors = 0
    neighbors.each do |neighbor|
      if neighbor
        live_neighbors += 1
      end
    end
    # Update status based on number of live_neighbors
    if population[i] == 0
      next_gen[i] = live_neighbors == 3
    else
      next_gen[i] = live_neighbors == 2 || live_neighbors == 3
    end
  end
  # Recursive call
  live(next_gen, rows, cols, gens - 1)
end

# Clears terminal window
def clear_screen
  system( "clear" ) || system( "cls" )
end

# Start simulation
start