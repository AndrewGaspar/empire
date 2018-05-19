#ifndef EMPIRE_MPI_H
#define EMPIRE_MPI_H

// Platform Differences
#if _WIN32
#include <vcruntime.h>

#define EMPIRE_IMPORT __declspec(dllimport)
#else
#define EMPIRE_IMPORT
#endif

// MPI Types
typedef struct empire_comm_t *MPI_Comm;
typedef struct empire_info_t *MPI_Info;

// Error classes
enum {
    MPI_SUCCESS = 0,
    MPI_ERR_BUFFER,
    MPI_ERR_COUNT,
    MPI_ERR_TYPE,
    MPI_ERR_TAG,
    MPI_ERR_COMM,
    MPI_ERR_RANK,
    MPI_ERR_REQUEST,
    MPI_ERR_ROOT,
    MPI_ERR_GROUP,
    MPI_ERR_OP,
    MPI_ERR_TOPOLOGY,
    MPI_ERR_DIMS,
    MPI_ERR_ARG,
    MPI_ERR_UNKNOWN,
    MPI_ERR_TRUNCATE,
    MPI_ERR_OTHER,
    MPI_ERR_INTERN,
    MPI_ERR_PENDING,
    MPI_ERR_IN_STATUS,
    MPI_ERR_ACCESS,
    MPI_ERR_AMODE,
    MPI_ERR_ASSERT,
    MPI_ERR_BAD_FILE,
    MPI_ERR_BASE,
    MPI_ERR_CONVERSION,
    MPI_ERR_DISP,
    MPI_ERR_DUP_DATAREP,
    MPI_ERR_FILE_EXISTS,
    MPI_ERR_FILE_IN_USE,
    MPI_ERR_FILE,
    MPI_ERR_INFO_KEY,
    MPI_ERR_INFO_NOKEY,
    MPI_ERR_INFO_VALUE,
    MPI_ERR_INFO,
    MPI_ERR_IO,
    MPI_ERR_KEYVAL,
    MPI_ERR_LOCKTYPE,
    MPI_ERR_NAME,
    MPI_ERR_NO_MEM,
    MPI_ERR_NOT_SAME,
    MPI_ERR_NO_SPACE,
    MPI_ERR_NO_SUCH_FILE,
    MPI_ERR_PORT,
    MPI_ERR_QUOTA,
    MPI_ERR_READ_ONLY,
    MPI_ERR_RMA_ATTACH,
    MPI_ERR_RMA_CONFLICT,
    MPI_ERR_RMA_RANGE,
    MPI_ERR_RMA_SHARED,
    MPI_ERR_RMA_SYNC,
    MPI_ERR_RMA_FLAVOR,
    MPI_ERR_SERVICE,
    MPI_ERR_SIZE,
    MPI_ERR_SPAWN,
    MPI_ERR_UNSUPPORTED_DATAREP,
    MPI_ERR_UNSUPPORTED_OPERATION,
    MPI_ERR_WIN,
    MPI_T_ERR_CANNOT_INIT,
    MPI_T_ERR_NOT_INITIALIZED,
    MPI_T_ERR_MEMORY,
    MPI_T_ERR_INVALID,
    MPI_T_ERR_INVALID_INDEX,
    MPI_T_ERR_INVALID_ITEM,
    MPI_T_ERR_INVALID_SESSION,
    MPI_T_ERR_INVALID_HANDLE,
    MPI_T_ERR_INVALID_NAME,
    MPI_T_ERR_OUT_OF_HANDLES,
    MPI_T_ERR_OUT_OF_SESSIONS,
    MPI_T_ERR_CVAR_SET_NOT_NOW,
    MPI_T_ERR_CVAR_SET_NEVER,
    MPI_T_ERR_PVAR_NO_WRITE,
    MPI_T_ERR_PVAR_NO_STARTSTOP,
    MPI_T_ERR_PVAR_NO_ATOMIC,
    MPI_ERR_LASTCODE
};

// Defined constants
enum { MPI_MAX_PORT_NAME = 256 };

#define MPI_ARGV_NULL 0

// MPI Routines
#ifdef __cplusplus
extern "C" {
#endif

// Global Variables
EMPIRE_IMPORT MPI_Comm MPI_COMM_SELF;
EMPIRE_IMPORT MPI_Comm MPI_COMM_WORLD;
EMPIRE_IMPORT MPI_Comm MPI_COMM_NULL;

EMPIRE_IMPORT MPI_Info MPI_INFO_NULL;

// Library initialization
EMPIRE_IMPORT int MPI_Init(int *argc, char ***argv);

#ifdef _WIN32
EMPIRE_IMPORT int MPI_InitW(int *argc, wchar_t ***argv);
#endif

EMPIRE_IMPORT int MPI_Finalize();

// MPI_Comm routines
EMPIRE_IMPORT int MPI_Comm_rank(MPI_Comm comm, int *rank);
EMPIRE_IMPORT int MPI_Comm_size(MPI_Comm comm, int *size);
EMPIRE_IMPORT int MPI_Comm_test_inter(MPI_Comm comm, int *flag);

// Section 6
// Section 6.4
int MPI_Comm_free(MPI_Comm *comm);

// Section 10
// Port routines
EMPIRE_IMPORT int MPI_Open_port(MPI_Info info, char *port_name);
EMPIRE_IMPORT int MPI_Close_port(char *port_name);

// Section 10.3
EMPIRE_IMPORT
int
MPI_Comm_spawn(
    const char *command,
    char *argv[],
    int maxprocs,
    MPI_Info info,
    int root,
    MPI_Comm comm,
    MPI_Comm *intercomm,
    int array_of_errcodes[]);

#ifdef _WIN32
EMPIRE_IMPORT
int
MPI_Comm_spawnW(
    const wchar_t *command,
    wchar_t *argv[],
    int maxprocs,
    MPI_Info info,
    int root,
    MPI_Comm comm,
    MPI_Comm *intercomm,
    int array_of_errcodes[]);
#endif

EMPIRE_IMPORT int MPI_Comm_get_parent(MPI_Comm *parent);

EMPIRE_IMPORT
int
MPI_Comm_spawn_multiple(
    int count,
    char *array_of_commands[],
    char **array_of_argv[],
    const int array_of_maxprocs[],
    const MPI_Info array_of_info[],
    int root,
    MPI_Comm comm,
    MPI_Comm *intercomm,
    int array_of_errcodes[]);
    
#ifdef _WIN32
EMPIRE_IMPORT
int
MPI_Comm_spawn_multipleW(
    int count,
    wchar_t *array_of_commands[],
    wchar_t **array_of_argv[],
    const int array_of_maxprocs[],
    const MPI_Info array_of_info[],
    int root,
    MPI_Comm comm,
    MPI_Comm *intercomm,
    int array_of_errcodes[]);
#endif

#ifdef __cplusplus
}
#endif

#endif // EMPIRE_MPI_H