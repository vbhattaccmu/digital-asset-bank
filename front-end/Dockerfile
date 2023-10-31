# Use an official Node.js runtime as the base image
FROM node:latest

# Set the working directory in the container
WORKDIR /home

# Copy package.json and package-lock.json to the container
COPY package*.json ./home/

# Copy the rest of the application files to the container
COPY . /home/

# Install project dependencies
RUN npm install

# Expose the port that the app will run on
EXPOSE 3000

# Run the app
CMD ["npm", "start"]
