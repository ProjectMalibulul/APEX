package service

import (
	"example.com/apex/repository"
)

type UserService struct {
	repository repository.UserRepository
}

type UserReader interface {
	FindUser(id string) string
}

func NewUserService(repository repository.UserRepository) UserService {
	return UserService{repository: repository}
}

