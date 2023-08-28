import os
import bcrypt
from cryptography.fernet import Fernet

while True:
    #Log in screen
    print('Welcome to Password Locker. Please choose an option below:')
    print('1: Log in')
    print('2: Create an account')
    print('3: Exit')

    usrInput = input()
    while usrInput != '1' and usrInput != '2' and usrInput != '3':
        print('Please type either 1, 2, or 3')
        usrInput = input()

    #Login
    if usrInput == '1':
        if not os.path.exists('bin.txt'):
            print("Can't login since an account has not been created yet")
            continue
        
        while True:
            print('Please enter the account password: ')
            usrInput = input()
            if usrInput != '':
                break
        
        usrInput = usrInput.encode('utf-8')
        salt = ''
        hashedPW = ''

        with open('bin.txt', 'r') as f:
            foundSalt = False
            foundHash = False
            for line in f:
                if 'salt' in line:
                    salt = line[7:].rstrip().encode('utf-8')
                    foundSalt = True
                elif 'hash' in line:
                    hashedPW = line[7:].rstrip().encode('utf-8')
                    foundHash = True
                if foundSalt and foundHash:
                    break
            
            if not foundSalt or not foundHash:
                print('Error finding salt or hash. Returning to login screen')
                continue

        tempHash = bcrypt.hashpw(usrInput, salt)
        if tempHash != hashedPW:
            print('Password incorrect')
            continue
        else:
            print('Login successful')


    #Account creation
    elif usrInput == '2':
        print("Please enter a master password. This will need to be complex as it will protect all of your passwords")
        password = input().encode('utf-8')
        salt = bcrypt.gensalt()
        hashedPW = bcrypt.hashpw(password, salt)

        #Create bin.txt and store salt and hash
        with open('bin.txt', 'a') as f:
            f.write('salt = ' + salt.decode('utf-8') + '\n')
            f.write('hash = ' + hashedPW.decode('utf-8') + '\n')
        
        print("Master password successfully created")

    #Exit
    elif usrInput == '3':
        break



    #Main password screen
    #for password in passwordFile:
        #print(password)
    print('1: Add password')
    print('2: Delete password')
    print('3: Logout')

    usrInput = input()
    while usrInput != '1' and usrInput != '2' and usrInput != '3':
        print('Please type either 1, 2, or 3')
        usrInput = input()

    #Add a password
    #if usrInput == '1':
    #print('Enter password to store: ')
    #usrInput = input()

    #Delete a password
    #elif usrInput == '2':

    #Go back to login if 3

print()