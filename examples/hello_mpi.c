#include <mpi.h>
#include <stdio.h>

int main(int argc, char **argv) {
    MPI_Init(&argc, &argv);

    int self_rank, self_size;
    MPI_Comm_rank(MPI_COMM_SELF, &self_rank);
    MPI_Comm_size(MPI_COMM_SELF, &self_size);

    printf("MPI_COMM_SELF { rank = %d, size = %d }\n", self_rank, self_size);

    int world_rank, world_size;
    MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
    MPI_Comm_size(MPI_COMM_WORLD, &world_size);

    printf("MPI_COMM_WORLD { rank = %d, size = %d }\n", world_rank, world_size);

    MPI_Finalize();
}