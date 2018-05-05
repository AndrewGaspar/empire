#include <mpi.h>
#include <stdio.h>
#include <assert.h>

int main() {
    MPI_Init(NULL, NULL);

    char port_name[MPI_MAX_PORT_NAME];
    MPI_Open_port(MPI_INFO_NULL, port_name);

    printf("Port: %s\n", port_name);

    int result = MPI_Close_port(port_name);
    if (result != MPI_SUCCESS) {
        printf("Error: MPI_Close_port result = %d\n", result);
    }

    MPI_Finalize();
}