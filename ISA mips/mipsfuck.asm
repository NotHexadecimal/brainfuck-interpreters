# made by Rampeo Mattone using MARS
# consider using the same emulator to run tests

# CONVENZIONI
# il registro $s0 punta all'istruzione a cui noi facciamo riferimento
# il registro $s1 punta alla cella della memoria su cui bf opera
# la memoria del programma bf viene collocata nei dati globali con etichetta MEM
# le istruzioni del programma sono salvate sullo stack mediante una syscall di lettura

.data
MEM: .space 30000	# 30k byte di memoria

.globl MAIN

.text
MAIN:
	li $v0, 5		# syscall readint
	syscall			# chiedo quanto grande e' l'input (restituito in $v0)
	addi $a1, $v0, 1	# imposto la grandezza del buffer per la syscall di lettura (sommo 1 per il byte \0 finale)
	# lo stack non viene indicizzato al byte ma alla parola
	addiu $sp, $sp, 3	# mi allineo al primo byte non usato dello stack
	subu $sp, $sp, $a1	# sposto lo stack pointer fino ad allocare abbastanza byte per l'intera lettura del codice (ora punta alla prima cella non usata per le istruzioni)
	addiu $a0, $sp, 1	# carico l'indirizzo di inizio istruzioni (inizio buffer per la syscall)
	li $v0, 8		# syscall readstring
	syscall			# chiedo di leggere il codice
	# adesso il codice e' nello stack e va interpretato
	# entro in un loop di lettura per eseguire il codice inserito
	addiu $s0, $sp, 1	# inizializzo il puntatore alle istruzioni
	la $s1, MEM		# carico l'indirizzo della cella di memoria a indice 0
	I_LOOP:			# inizio il ciclo di interpretazione
		lb $t0, ($s0)		# carico l' istruzione corrente
		# controllo se l'istruzione ha senso (e vado a eseguirla) o se va saltata
		beq $t0, '+', INC_V		# incrementa il valore della cella di memoria
		beq $t0, '-', DEC_V		# decrementa il valore della cella di memoria
		beq $t0, '>', INC_P		# incrementa il puntatore alla memoria
		beq $t0, '<', DEC_P		# decrementa il puntatore alla memoria
		beq $t0, ',', INPUT		# richiede input dall'utente
		beq $t0, '.', OUTPUT		# fa un output all'utente
		beq $t0, '[', JUMP_F		# salta avanti fino alla parentesi complementare se il valore della memoria e' 0
		beq $t0, ']', JUMP_B		# salta indietro fino alla parentesi complementare se il valore della memoria e' diverso da 0
		beq $t0, '\0', EXIT		# se viene letto un null byte allora e' finita la stringa di codice e devo uscire dal programma
		END_ISTR:			# se sono qui e' perche' il carattere non e' una istruzione, oppure perche' l'istruzione e' stata eseguita
		addiu $s0, $s0, 1		# passo all'istruzione successiva
		j I_LOOP			# e ricomincio il ciclo

	EXIT:			# esco dal programma
	li $v0, 10
	syscall
	
INC_V:	# incrementa il valore della cella selezionata
	lb  $t0, ($s1)
	addiu $t0, $t0, 1
	sb $t0, ($s1)
	j END_ISTR

DEC_V:	# decrementa il valore della cella selezionata
	lb $t0, ($s1)
	addiu $t0, $t0, -1
	sb $t0, ($s1)
	j END_ISTR

INC_P:	# incrementa il puntatore
	addiu $s1, $s1, 1
	j END_ISTR

DEC_P:	# decrementa il puntatore
	addiu $s1, $s1, -1
	j END_ISTR

JUMP_F:	# salta avanti se la memoria e' a 0
	lb $t0, ($s1)	# carica in $t0 il valore nella cella selezionata
	bnez $t0, END_ISTR	# se non devo saltare avanti, passo alla prossima istruzione
	move $t1, $zero		# $t1 ora conta il numero di parentesi aperte, da chiudere nuovamente prima di uscire dal loop
	LOOP_FORWARD:
		addiu $s0, $s0, 1		# prendo la prossima istruzione
		lb $t0, ($s0)			# $t0 ora tiene salvata l'istruzione codificata
		beq $t0, '[', JUMP_F_INC	# se l'istruzione e' una [ allora devo aumentare il contatore di parentesi da chiudere
		beq $t0, ']', JUMP_F_DEC	# se l'istruzione e' una ] allora devo decrementare il contatore di parentesi da chiudere, oppure ho concluso il loop
		j LOOP_FORWARD			# se l'istruzione non e' una [ o una ] allora passo alla successiva
	
	JUMP_F_INC:
		addi $t1, $t1, 1
		j LOOP_FORWARD
		
	JUMP_F_DEC:
		beqz $t1, END_ISTR
		addi $t1, $t1, -1
		j LOOP_FORWARD
		
JUMP_B:	# salta avanti se la memoria e' a 0
	lb $t0, ($s1)	# carica in $t0 il valore nella cella selezionata
	beqz $t0, END_ISTR	# se non devo saltare avanti, passo alla prossima istruzione
	move $t1, $zero		# $t1 ora conta il numero di parentesi aperte, da chiudere nuovamente prima di uscire dal loop
	LOOP_BACKWARDS:
		addiu $s0, $s0, -1		# prendo la prossima istruzione
		lb $t0, ($s0)			# $t0 ora tiene salvata l'istruzione codificata
		beq $t0, '[', JUMP_B_DEC	# se l'istruzione e' una [ allora devo decrementare il contatore di parentesi da chiudere, oppure ho concluso il loop
		beq $t0, ']', JUMP_B_INC	# se l'istruzione e' una ] allora devo aumentare il contatore di parentesi da chiudere
		j LOOP_BACKWARDS		# se l'istruzione non e' una [ o una ] allora passo alla successiva
	
	JUMP_B_INC:
		addi $t1, $t1, 1
		j LOOP_BACKWARDS
		
	JUMP_B_DEC:
		beqz $t1, END_ISTR
		addi $t1, $t1, -1
		j LOOP_BACKWARDS

OUTPUT:	# syscall to print (11)
	lb $a0, ($s1)
	li $v0, 11
	syscall
	j END_ISTR

INPUT:	# syscall to read char (12)
	li $v0, 12
	syscall
	sb $v0, ($s1)
	j END_ISTR
