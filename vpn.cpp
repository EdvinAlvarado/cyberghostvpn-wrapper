#include <cstddef>
#include <iostream>
#include <iterator>
#include <ostream>
#include <string>
#include <vector>
#include <map>

enum return_status {OK, PROGRAM_UNINSTALLED, WRONG_INPUT, WRONG_COMMAND, WRONG_CITY};

int main(int argc, char* argv[]) {
	if (system("pacman -Qq cyberghostvpn")) {std::cerr << "cyberghostvpn is not installed" << std::endl; return PROGRAM_UNINSTALLED;}

	std::vector<std::string> args(argv, argv+argc);
	std::string cmd = "";
	const std::map<std::string, std::string> one = {{"stop", " --stop"}, {"help", " --help"}, {"status", " --status"}, {"connect", " --traffic --connect"}};
	for (size_t i = 0; i < args.size(); i++) {
		switch (i) {
			case 0: {
				cmd.append("cyberghostvpn");
				break;
			}
			case 1: {
				auto one_it = one.find(args[1]);
				if (one_it != one.end()) {cmd.append(one_it->second);} else {return WRONG_INPUT;}
				break;
			}
			case 2: {
				if (args[1] == "connect") {
					if (args[2].length() == 2) {cmd.append(" ").append(args[2]);}
					else {std::cerr << "Wrong country code" << std::endl; return WRONG_INPUT;}
				}
				else {std::cerr << "unsupported extra input" << std::endl; return WRONG_INPUT;}
				break;
			}
			case 3: {
				if (args[1] == "connect") {cmd.append(" ").append(args[3]);}
				else {std::cerr << "unsupported input" << std::endl; return WRONG_INPUT;}
				break;
			}
			default:
				std::cerr << "too many inputs" << std::endl;
				return WRONG_INPUT;
		}
	}
	if (system(cmd.c_str())) {std::cerr << "wrong command" << std::endl; return WRONG_COMMAND;}
	else {return OK;}
}

