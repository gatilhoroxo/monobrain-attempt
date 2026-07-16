with open('temas.txt', 'r', encoding='utf-8') as arquivo:
  linhas = arquivo.readlines()

for linha in linhas:
  linha_limpa = linha.strip()
  partes = linha_limpa.split('|')
  print(f"Tema:{partes[0]} - Pergunta: {partes[1]}")
  