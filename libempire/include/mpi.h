#ifndef EMPIRE_MPI_H
#define EMPIRE_MPI_H

#if _WIN32
#include <vcruntime.h>

#define EMPIRE_EXTERN __declspec(dllimport)
#else
#define EMPIRE_EXTERN extern
#endif

// MPI Types
typedef struct empire_comm_t *MPI_Comm;

#ifdef __cplusplus
extern "C" {
#endif

// Global Members
EMPIRE_EXTERN MPI_Comm MPI_COMM_SELF;
EMPIRE_EXTERN MPI_Comm MPI_COMM_WORLD;

// Library initialization
int MPI_Init(int *argc, char ***argv);

#ifdef _WIN32
int MPI_InitW(int *argc, wchar_t ***argv);
#endif

int MPI_Finalize();

// MPI_Comm routines
int MPI_Comm_rank(MPI_Comm comm, int *rank);
int MPI_Comm_size(MPI_Comm comm, int *size);

#ifdef __cplusplus
}
#endif

#endif // EMPIRE_MPI_H