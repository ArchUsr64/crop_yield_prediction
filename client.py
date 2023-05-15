from socket import socket
from tkinter import *
main = Tk()
main.title("Crop Yield Prediction")
main.geometry("350x150")
l = Label(text="Crop")
l.grid(row=0, column=0)
options = ["Maize", "Potatoes", "Sorghum", "Soybeans", "Wheat", "Cassava", "Sweet Potatoes", "Plantatins and others", "Yams"]
clicked = StringVar()
clicked.set("Wheat")
crop = OptionMenu(main, clicked, *options)
crop.grid(row=0, column=1)
l = Label(text="Average Rainfall")
l.grid(row=1, column=0)
rainfall = Entry()
rainfall.grid(row=1, column=1)
l = Label(text="Pesticide")
l.grid(row=2, column=0)
pesticide = Entry()
pesticide.grid(row=2, column=1)
l = Label(text="Average Temperature")
l.grid(row=3, column=0)
temperature = Entry()
temperature.grid(row=3, column=1)
result = Label()
def send():
    s = socket()
    s.connect(("localhost", 1234))
    crop_index = options.index(clicked.get())
    data = f"{crop_index} {rainfall.get()} {pesticide.get()} {temperature.get()}\n".encode()
    print(data)
    s.send(data)
    result.config(text=f"{s.recv(1024).decode()} Tons/Hectare")
button = Button(text="Predict", command=send)
button.grid(row=4, column=0)
result.grid(row=4, column=1)
main.mainloop()
