#!/usr/bin/env python
import subprocess
import sys

assert sys.version_info >= (3, 10) 

def main():
    def vpn_installed():
        process = subprocess.Popen(['pacman', '-Qq', "cyberghostvpn"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        _,err = process.communicate()
        return err==b''

    if vpn_installed():
        args = sys.argv
        cmd = []
        for i, arg in enumerate(args):
            match i:
                case 0:
                    cmd.append('cyberghostvpn')
                case 1:
                    one = {'stop':['--stop'], 'status':['--status'], 'help':['--help'], 'connect':['--traffic', '--connect', '--country-code']}
                    try:
                        cmd.extend(one[arg])
                    except KeyError:
                        raise Exception("command cannot be found")
                case 2:
                    if '--country-code' == cmd[-1]:
                        if len(arg) == 2:
                            cmd.append(arg)
                        else:
                            raise Exception("wrong country code")
                case 3:
                    cmd.append('--city')
                    cmd.append(f'{args[i]}')
    else:
        raise Exception("cyberghostvpn is not installed or not using pacman")

    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    print(f'{process.communicate()}')

if __name__ == "__main__":
    main()
