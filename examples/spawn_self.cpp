#include <mpi.h>
#include <array>

int main(int argc, char **argv) {
    MPI_Init(&argc, &argv);

    if (argc < 2) {
        printf_s("Hello from the parent!\n");

        MPI_Comm intercomm;
        int errors[2];

        std::array<char*,2> commands{argv[0], argv[0]};

        std::array<char*,2> left_command_args{"Rod", MPI_ARGV_NULL};
        std::array<char*,2> right_command_args{"Steve", MPI_ARGV_NULL};

        std::array<char**, 2> argvs{
            left_command_args.data(),
            right_command_args.data()
        };

        std::array<int, 2> maxprocs{1, 1};
        std::array<MPI_Info, 2> infos{MPI_INFO_NULL, MPI_INFO_NULL};
        
        MPI_Comm_spawn_multiple(
            2,
            commands.data(),
            argvs.data(),
            maxprocs.data(),
            infos.data(),
            0,
            MPI_COMM_SELF,
            &intercomm,
            errors);

        if (intercomm == MPI_COMM_NULL) {
            printf("intercomm shouldn't be null!");
            std::exit(EXIT_FAILURE);
        }

        MPI_Comm_free(&intercomm);

        if (intercomm != MPI_COMM_NULL) {
            printf("intercomm should be null!");
            std::exit(EXIT_FAILURE);
        }
    } else {
        printf_s("Hello from %s\n", argv[1]);
    }

    MPI_Finalize();
}