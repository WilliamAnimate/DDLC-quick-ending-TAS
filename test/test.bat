@echo off

:: set sayori and stuff
cd characters
ren s sayori.chr
cd..

if exist test.exe (
	test.exe
) else (
	if exist ddlc_quick_ending_tas.exe (
		ddlc_quick_ending_tas.exe
	) else (
		echo no test.exe nor ddlc_quick_ending_tas.exe, exiting.
	)
)