import os
import base64
import bcrypt
from cryptography.fernet import Fernet

while True:
    # Log in screen
    print('\nWelcome to Password Locker. Please choose an option below:')
    print('1: Log in')
    print('2: Create an account')
    print('3: Exit')

    usrInput = input()
    masterPass = ""
    while usrInput != '1' and usrInput != '2' and usrInput != '3':
        print('Please type either 1, 2, or 3')
        usrInput = input()

    # Login
    if usrInput == '1':
        if not os.path.exists('bin.txt'):
            print("Can't login since an account has not been created yet")
            continue
        
        while True:
            print('\nPlease enter the account password: ')
            usrInput = input()
            if usrInput != '':
                break
        
        usrInput = usrInput.encode('utf-8')
        masterPass = usrInput
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
            print('Login successful\n')

    # Account creation
    elif usrInput == '2':
        print("Please enter a master password. This will need to be complex as it will protect all of your passwords")
        password = input().encode('utf-8')
        salt = bcrypt.gensalt()
        hashedPW = bcrypt.hashpw(password, salt)

        # Create bin.txt and store salt and hash
        with open('bin.txt', 'a') as f:
            f.write('salt = ' + salt.decode('utf-8') + '\n')
            f.write('hash = ' + hashedPW.decode('utf-8') + '\n')
        
        masterPass = password
        print("Master password successfully created")

    # Exit
    elif usrInput == '3':
        break

    # Main password screen
    # masterPass has to be a string with exactly 32 bytes
    while len(masterPass) < 32:
        masterPass += b'='
    masterPass = masterPass[:32]   
    
    cipher = Fernet(base64.urlsafe_b64encode(masterPass))

    while True:
        # Print passwords
        passwords = []
        with open('bin.txt', 'r') as f:
            for line in f:
                if 'salt' in line or 'hash' in line:
                    continue
                p = cipher.decrypt(line.encode('utf-8')).decode('utf-8')
                passwords.append(p)
                print(p)

        print('1: Add password')
        print('2: Delete password')
        print('3: Logout')
        usrInput = input()

        while usrInput != '1' and usrInput != '2' and usrInput != '3':
            print('Please type either 1, 2, or 3')
            usrInput = input()

        # Add a password
        if usrInput == '1':
            print('\nEnter password to store: ')
            usrInput = input()
            
            encrypted = cipher.encrypt(usrInput.encode('utf-8'))
            with open('bin.txt', 'a') as f:
                f.write(encrypted.decode('utf-8') + '\n')

            print("Password stored\n")

        # Delete a password
        elif usrInput == '2':
            print("\nSelect which password to delete:")
            
            # print list of passwords
            counter = 1
            for p in passwords:
                print('{}: {}'.format(counter, p))
                counter += 1
            print('{}: Cancel'.format(counter))
            usrInput = input()

            while int(usrInput) < 1 and int(usrInput) > counter:
                print('Please choose one of the options displayed')
                usrInput = input()

            if int(usrInput) == counter:
                print()
                continue

            # remove encrypted password from file
            lineNum = int(usrInput) + 1
            with open('bin.txt' , "r") as f:
                fileLines = f.readlines()
            
            del fileLines[lineNum]

            with open('bin.txt' , "w") as f:
                f.writelines(fileLines)
            
            print('\nPassword successfully removed\n')

        # Go back to log in if 3
        elif usrInput == '3':
            break

print()