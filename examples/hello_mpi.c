#include <mpi.h>
#include <stdio.h>

int main(int argc, char **argv) {
    int value = MPI_Init(&argc, &argv);
    if (value != MPI_SUCCESS)
        printf("MPI_Init failed: status = %d\n", value);

    int self_rank, self_size, self_flag;
    MPI_Comm_rank(MPI_COMM_SELF, &self_rank);
    MPI_Comm_size(MPI_COMM_SELF, &self_size);
    MPI_Comm_test_inter(MPI_COMM_SELF, &self_flag);

    printf("MPI_COMM_SELF { rank = %d, size = %d, is_intercomm = %d }\n", self_rank, self_size, self_flag);

    int world_rank, world_size, world_flag;
    MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
    MPI_Comm_size(MPI_COMM_WORLD, &world_size);
    MPI_Comm_test_inter(MPI_COMM_SELF, &world_flag);

    printf("MPI_COMM_WORLD { rank = %d, size = %d, is_intercomm = %d }\n", world_rank, world_size, world_flag);

    MPI_Finalize();
}