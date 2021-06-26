#include <iostream>
#include <string>
#include <vector>

enum return_status {OK, PROGRAM_UNINSTALLED, WRONG_INPUT, WRONG_COMMAND, WRONG_CITY};

int main(int argc, char* argv[]) {
	std::vector<std::string> args(argv, argv+argc);
	if (system("pacman -Qq cyberghostvpn")) {std::cerr << "cyberghostvpn is not installed" << std::endl; return PROGRAM_UNINSTALLED;}
	switch (argc) {
		case 2:
			if 		(args[1] == "stop") {system("cyberghostvpn --stop"); return OK;}
			else if (args[1] == "help") {system("cyberghostvpn --help"); return OK;}
			else if (args[1] == "status") {system("cyberghostvpn --status"); return OK;}
			break;	
		case 3:
			if (args[1] == "connect") {
				std::string strC = "cyberghostvpn --traffic --connect --country-code ";
				if (args[2].length() == 2) {
					strC.append(args[2]);
					system(strC.c_str());
					return OK;
				}
				else {std::cerr << "Wrong country code" << std::endl; return WRONG_INPUT;}
			}
			break;
		case 4:
			if (args[1] == "connect") {
				std::string strC = "cyberghostvpn --traffic --connect --country-code ";
				if (args[2].length() == 2) {strC.append(args[2]);}
				else {std::cerr << "Wrong country code" << std::endl; return WRONG_INPUT;}
				
				strC.append(" --city '" + args[3] + "'");
				if (system(strC.c_str())) {std::cerr << "Wrong city" << std::endl; return WRONG_CITY;}
				else {return OK;}
				// return (system(strC.c_str())) ? OK : WRONG_CITY;
			}
			break;
	}
	std::cerr << "wrong command" << std::endl;
	return WRONG_COMMAND;
}
