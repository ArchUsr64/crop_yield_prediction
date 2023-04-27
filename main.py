import tkinter as tk
import pandas as pd
import numpy as np
from sklearn.compose import ColumnTransformer
from sklearn.preprocessing import OneHotEncoder
from sklearn.ensemble import RandomForestRegressor
import joblib
import seaborn as sns

# Load the CSV file into a Pandas DataFrame
data = pd.read_csv('yield_df.csv')

# Split the data into input features (X) and output variable (y)
X = data[['Area', 'Item', 'pesticides_tonnes', 'avg_temp', 'average_rain_fall_mm_per_year']]
y = data['hg/ha_yield']

# Create a ColumnTransformer to one-hot encode the categorical variables
ct = ColumnTransformer(transformers=[('encoder', OneHotEncoder(), [0, 1])])
X = ct.fit_transform(X)

# Train a random forest regressor on the data
print("Loading the regressor parameters...")
regressor = joblib.load("./model.joblib")


# Define function to predict yield based on input parameters
def predict_yield(country, crop_type, pesticide_used, avg_temperature, avg_rainfall):
    input_data = [[country, crop_type, pesticide_used, avg_temperature, avg_rainfall]]
    input_data = ct.transform(input_data)
    predicted_yield = regressor.predict(input_data)[0]
    result_label.config(text=f"Estimated yield: {predicted_yield:.2f} tonnes")

# Create the GUI window
window = tk.Tk()
window.title("Crop Yield Prediction")
window.geometry("400x400")

# Create the input fields
country_label = tk.Label(window, text="Country:")
country_label.pack()
country_entry = tk.Entry(window)
country_entry.pack()

crop_label = tk.Label(window, text="Crop Type:")
crop_label.pack()
crop_entry = tk.Entry(window)
crop_entry.pack()

pesticide_label = tk.Label(window, text="Pesticide Used (in tonnes):")
pesticide_label.pack()
pesticide_entry = tk.Entry(window)
pesticide_entry.pack()

temp_label = tk.Label(window, text="Average Temperature (in deg C):")
temp_label.pack()
temp_entry = tk.Entry(window)
temp_entry.pack()

rainfall_label = tk.Label(window, text="Average Rainfall (in mm per year):")
rainfall_label.pack()
rainfall_entry = tk.Entry(window)
rainfall_entry.pack()

# Create the predict button
predict_button = tk.Button(window, text="Predict", command=lambda: predict_yield(country_entry.get(), crop_entry.get(), float(pesticide_entry.get()), float(temp_entry.get()), float(rainfall_entry.get())))
predict_button.pack()

# Create the result label
result_label = tk.Label(window, text="Enter input and click Predict")
result_label.pack()

# Run the GUI
window.mainloop()

# Predict the hg/ha_yield for a given input
# Area, Crop, Pesticides Used, Avg Temperature, Avg Rainfall
new_data = [['India', 'Wheat', 121.0, 16.37, 1485.0]]
new_data = ct.transform(new_data)
predicted_yield = regressor.predict(new_data)[0]

print(f"Estimated yield: {predicted_yield:.2f} tonnes")
