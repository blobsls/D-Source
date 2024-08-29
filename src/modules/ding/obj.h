#ifndef UNLETTED_OBJECT_H
#define UNLETTED_OBJECT_H

#include <string>
#include <unordered_map>

class UnlettedObject {
public:
    UnlettedObject();
    ~UnlettedObject();

    void setProperty(const std::string& key, const std::string& value);
    std::string getProperty(const std::string& key) const;
    bool hasProperty(const std::string& key) const;
    void removeProperty(const std::string& key);

private:
    std::unordered_map<std::string, std::string> properties;
};

#endif // UNLETTED_OBJECT_H
