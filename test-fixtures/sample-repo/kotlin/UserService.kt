package apex.service

import apex.data.UserRepository

interface UserReader {
    fun findUser(id: String): String?
}

class UserService(private val repository: UserRepository) : UserReader {
    override fun findUser(id: String): String? = repository.find(id)
}

object UserModule

