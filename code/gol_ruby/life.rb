def life ( rows, cols, some, gens )
  population = new Array( rows * cols )
  population.each do |person|
    person = rand( max= 1 ) < some
  end
  live( population, rows, cols, gens )
end

def live ( population, rows, cols, gens )
  # Base case
  if gens < 1
    return 0
  end
  # Print status of current gen
  printf( "Generation %3d\n", gens )
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

# Start simulation
life(50, 20, 0.619, 200 )