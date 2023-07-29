#include <iostream>
#include <windows.h>
#include <tlhelp32.h>

void SetProcessPriority(const wchar_t* processName, DWORD priority) {
    PROCESSENTRY32 procEntry;
    HANDLE snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

    if (snapshot == INVALID_HANDLE_VALUE) {
        return;
    }

    procEntry.dwSize = sizeof(PROCESSENTRY32);

    while (Process32Next(snapshot, &procEntry) == TRUE) {
        if (_wcsicmp(procEntry.szExeFile, processName) == 0) {
            HANDLE process = OpenProcess(PROCESS_SET_INFORMATION, TRUE, procEntry.th32ProcessID);
            SetPriorityClass(process, priority);
            CloseHandle(process);
        }
    }

    CloseHandle(snapshot);
}

int main() {
    const wchar_t* processName = L"DDLC.exe";
    DWORD priority = HIGH_PRIORITY_CLASS;

    SetProcessPriority(processName, priority); // Windows indirectly increases I/O priority for processes that are high priority, according to my research.
                                               // I wouldn't trust that info with my life, but its not like I found any code that can do this.
                                               // eh, i'll read systeminformer's source code later, its under MIT.

    return 0;
}
