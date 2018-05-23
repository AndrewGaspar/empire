#include <mpi.h>
#include <array>

int main(int argc, char **argv) {
    MPI_Init(&argc, &argv);

    std::array<MPI_Info, 3> infos;
    MPI_Info_create(&infos[0]);

    int valuelen = -1;
    int flag = 0;
    MPI_Info_get_valuelen(infos[0], "foo", &valuelen, &flag);

    if (flag) {
        printf("Incorrectly found value!\n");
        std::exit(EXIT_FAILURE);
    }

    MPI_Info_set(infos[0], "foo", "bar");

    for (auto i = 0; i < infos.size(); i++) {
        auto &info = infos[i];

        int nkeys = 0;
        MPI_Info_get_nkeys(info, &nkeys);
        if (nkeys != 1) {
            printf("Not the correct number of keys!\n");
            std::exit(EXIT_FAILURE);
        }

        char key[MPI_MAX_INFO_KEY];
        MPI_Info_get_nthkey(info, 0, key);

        if (0 != strcmp(key, "foo")) {
            printf("Got the wrong key!\n");
            std::exit(EXIT_FAILURE);
        }

        valuelen = -1;
        flag = 0;
        MPI_Info_get_valuelen(info, key, &valuelen, &flag);

        if (valuelen != strlen("foo")) {
            printf("Go the wrong valuelen!\n");
            std::exit(EXIT_FAILURE);
        }

        char value[4];
        MPI_Info_get(info, "foo", 4, value, &flag);

        if (!flag) {
            printf("Could not find value!\n");
            std::exit(EXIT_FAILURE);
        }

        if (0 != strcmp(value, "bar")) {
            printf("Did not get correct value!\n");
            std::exit(EXIT_FAILURE);
        }

        if (i < infos.size() - 1) {
            MPI_Info_dup(info, &infos[i + 1]);
        }

        MPI_Info_free(&info);
    }

    MPI_Finalize();
}