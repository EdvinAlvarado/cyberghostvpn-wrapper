#include <cstddef>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>
#include <map>

void error(std::string s) {std::cerr << s << std::endl;}

enum return_status {OK, PROGRAM_UNINSTALLED, WRONG_INPUT, WRONG_COMMAND, WRONG_CITY};

int main(int argc, char* argv[]) {
	if (system("pacman -Qq cyberghostvpn")) {error("cyberghostvpn is not installed"); return PROGRAM_UNINSTALLED;}

	std::vector<std::string> args(argv, argv+argc);
	std::string cmd = "";
	const std::map<std::string, std::string> one = {{"stop", " --stop"}, {"help", " --help"}, {"status", " --status"}, {"connect", " --traffic --connect"}};

	for (size_t i = 0; i < args.size(); i++) {
		switch (i) {
			case 0:
				cmd.append("cyberghostvpn");
				break;
			case 1: {
				auto one_it = one.find(args[1]);
				if (one_it != one.end()) {cmd.append(one_it->second);} else {return WRONG_INPUT;}
				break;
			}
			case 2:
				if (args[1] == "connect") {
					if (args[2].length() == 2) {cmd.append(" ").append(args[2]);}
					else {error("Wrong country code"); return WRONG_INPUT;}
				}
				else {error("unsupported extra input"); return WRONG_INPUT;}
				break;
			case 3:
				if (args[1] == "connect") {cmd.append(" ").append(args[3]);}
				else {error("unsupported input"); return WRONG_INPUT;}
				break;
			default:
				error("too many inputs");
				return WRONG_INPUT;
		}
	}
	if (system(cmd.c_str())) {error("wrong command"); return WRONG_COMMAND;}
	else {return OK;}
}
