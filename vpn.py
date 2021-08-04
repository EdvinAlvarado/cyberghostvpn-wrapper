#!/usr/bin/env python
import subprocess
import sys


def main():
    def vpn_installed():
        process = subprocess.Popen(['pacman', '-Qq', "cyberghostvpn"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        _,err = process.communicate()
        return err==b''

    if vpn_installed():
        args = sys.argv
        cmd = []
        for i in range(len(args)):
            if i==0:
                cmd.append('cyberghostvpn')
            elif i==1:
                one = {'stop':['--stop'], 'status':['--status'], 'help':['--help'], 'connect':['--traffic', '--connect', '--country-code']}
                try:
                    cmd.extend(one[args[i]])
                except KeyError:
                    raise Exception("command cannot be found")
            elif i==2:
                if '--country-code' ==  cmd[-1]:
                    if len(args[i]) == 2:
                        cmd.append(args[i])
                    else:
                        raise Exception("wrong country code")
            elif i==3:
                cmd.append('--city')
                cmd.append(f'{args[i]}')
    else:
        raise Exception("cyberghostvpn is not installed or not using pacman")

    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    print(f'{process.communicate()}')

if __name__ == "__main__":
    main()
